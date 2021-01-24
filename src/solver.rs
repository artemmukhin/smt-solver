// use daggy::Dag;
// use disjoint_sets::array::UnionFind;
// use petgraph::graph_impl::NodeIndex;
// use smt_solver::{Kind, Relation, Term};
use crate::{Kind, Relation, Term};
// use std::collections::hash::map::HashMap;
// use std::collections::hash::set::HashSet;
use std::collections::HashMap;
use std::collections::HashSet;

use daggy::petgraph::visit::NodeIndexable;
use daggy::{Dag, NodeIndex, Walker};
use disjoint_sets::UnionFind;

#[derive(Clone, Eq, PartialEq, Hash)]
struct NodeWrapper<'a>(&'a Term);

pub struct Solver<'a> {
    relations: Vec<&'a Relation>,
    subterms: Vec<&'a Term>,
    dag: Dag<NodeWrapper<'a>, ()>,
    union_find: UnionFind,
}

impl<'a> Solver<'a> {
    pub fn from(relations: Vec<&'a Relation>) -> Solver<'a> {
        let subterms = Solver::compute_subterms(&relations);
        let dag = Solver::compute_dag(&subterms);
        let union_find = UnionFind::new(subterms.len());

        Solver {
            relations,
            subterms,
            dag,
            union_find,
        }
    }

    fn split_relations(relations: &Vec<&'a Relation>) -> (Vec<&'a Relation>, Vec<&'a Relation>) {
        let (mut equal_relations, mut not_equal_relations) = (vec![], vec![]);

        for relation in relations {
            match relation.kind {
                Kind::Equal => equal_relations.push(*relation),
                Kind::NotEqual => not_equal_relations.push(*relation),
            }
        }

        (equal_relations, not_equal_relations)
    }

    fn compute_subterms(relations: &Vec<&'a Relation>) -> Vec<&'a Term> {
        let mut all_subterms: Vec<&Term> = vec![];

        let mut visited: HashSet<&Term> = HashSet::new();

        for relation in relations.iter() {
            println!("Relation: {}", relation);
            let subterms = relation.subterms().collect::<Vec<_>>();
            for subterm in subterms {
                if !visited.contains(subterm) {
                    all_subterms.push(subterm);
                    visited.insert(subterm);
                }
            }
        }
        println!();
        println!("Subterms:");
        for subterm in all_subterms.iter() {
            println!("{}", subterm);
        }
        println!();

        all_subterms
    }

    fn compute_dag<'b>(subterms: &Vec<&'b Term>) -> Dag<NodeWrapper<'b>, (), u32> {
        let mut dag = Dag::<NodeWrapper, (), u32>::new();
        let mut nodes: HashMap<&Term, NodeIndex> = HashMap::new();

        for subterm in subterms.iter() {
            let node = NodeWrapper(*subterm);
            let index = dag.add_node(node);
            nodes.insert(*subterm, index);
        }
        for subterm in subterms.iter() {
            let node = nodes[*subterm];
            let children = subterm
                .arguments
                .iter()
                .map(|arg| nodes[&**arg])
                .collect::<Vec<_>>();
            for child in children {
                let _ = dag.add_edge(node, child, ());
            }
        }
        dag
    }

    fn congruent(&self, node1: NodeIndex, node2: NodeIndex) -> bool {
        let node1_weight = self.dag.node_weight(node1).unwrap();
        let node2_weight = self.dag.node_weight(node2).unwrap();

        if node1_weight.0.name != node2_weight.0.name {
            return false;
        }
        let node1_children = self
            .dag
            .children(node1)
            .iter(&self.dag)
            .map(|(_, node)| node)
            .collect::<Vec<_>>();
        let node2_children = self
            .dag
            .children(node2)
            .iter(&self.dag)
            .map(|(_, node)| node)
            .collect::<Vec<_>>();
        if node1_children.len() != node2_children.len() {
            return false;
        }

        for (child1, child2) in node1_children.iter().zip(node2_children.iter()) {
            let term1_class = self.union_find.find(child1.index());
            let term2_class = self.union_find.find(child2.index());
            if term1_class != term2_class {
                return false;
            }
        }

        return true;
    }

    #[cfg(test)]
    pub fn find_all_congruent_terms(&self) -> Vec<String> {
        let mut result = vec![];
        for index1 in 0..self.subterms.len() {
            for index2 in index1 + 1..self.subterms.len() {
                let dag_index1 = self.dag.from_index(index1);
                let dag_index2 = self.dag.from_index(index2);
                if self.congruent(dag_index1, dag_index2) {
                    result.push(format!(
                        "{} ~ {}",
                        self.subterms[index1], self.subterms[index2]
                    ));
                }
            }
        }
        result
    }

    fn predecessors(&self, node: NodeIndex) -> Vec<NodeIndex> {
        self.dag
            .parents(node)
            .iter(&self.dag)
            .map(|(_, n)| n)
            .collect()
    }

    fn merge(&mut self, node1: NodeIndex, node2: NodeIndex) {
        println!(
            "{} ~ {}",
            self.subterms[node1.index()], self.subterms[node2.index()]
        );

        let index1 = node1.index();
        let index2 = node2.index();
        let node1_class = self.union_find.find(index1);
        let node2_class = self.union_find.find(index2);

        if node1_class != node2_class {
            let preds1 = self.predecessors(node1);
            let preds2 = self.predecessors(node2);

            self.union_find.union(index1, index2);

            for pred1 in preds1.iter() {
                for pred2 in preds2.iter() {
                    let index1 = pred1.index();
                    let index2 = pred2.index();
                    let pred1_class = self.union_find.find(index1);
                    let pred2_class = self.union_find.find(index2);
                    if pred1_class != pred2_class && self.congruent(*pred1, *pred2) {
                        // println!(
                        //     "Congruence found: {} ~ {}",
                        //     self.subterms[index1], self.subterms[index2]
                        // );
                        self.merge(*pred1, *pred2);
                    }
                }
            }
        }
    }

    pub fn check_satisfiable(&mut self) -> bool {
        let (equal_relations, not_equal_relations) = Solver::split_relations(&self.relations);


        println!("DAG edges:");
        for e in self.dag.raw_edges().iter() {
            println!("{} -> {}", self.subterms[e.source().index()], self.subterms[e.target().index()]);
        }
        println!();
        
        println!("Merging:");
        for relation in equal_relations {
            let index1 = self
                .subterms
                .iter()
                .position(|term| *term == &relation.left)
                .unwrap();
            let index2 = self
                .subterms
                .iter()
                .position(|term| *term == &relation.right)
                .unwrap();

            let dag_index1 = self.dag.from_index(index1);
            let dag_index2 = self.dag.from_index(index2);
            self.merge(dag_index1, dag_index2);
        }
        println!();
        
        let mut hm: HashMap<usize, Vec<&Term>> = HashMap::new();
        
        for (i, class) in self.union_find.to_vec().iter().enumerate() {
            match hm.get_mut(class) {
                None => { hm.insert(*class, vec![self.subterms[i]]); }
                Some(v) => { v.push(self.subterms[i]); }
            };
        }
        
        println!("Union-find:");
        for (k, v) in hm {
            print!("{{ ");
            for (i, st) in v.iter().enumerate() {
                print!("{}", st);
                if i != v.len() - 1 {
                    print!(", ");
                }
            }
            print!(" }}");
            println!();
        }
        println!();

        
        for relation in not_equal_relations {
            let index1 = self
                .subterms
                .iter()
                .position(|term| *term == &relation.left)
                .unwrap();
            let index2 = self
                .subterms
                .iter()
                .position(|term| *term == &relation.right)
                .unwrap();

            let class1 = self.union_find.find(index1);
            let class2 = self.union_find.find(index2);
            if class1 == class2 {
                return false;
            }
        }
        return true;
    }
}
