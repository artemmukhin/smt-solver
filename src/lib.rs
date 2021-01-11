use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::iter::Chain;

use daggy::petgraph::visit::NodeIndexable;
use daggy::{Dag, NodeIndex, Walker};
use disjoint_sets::UnionFind;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Term {
    pub name: String,
    pub arguments: Vec<Box<Term>>,
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // non-empty arguments
        if let Some((last, elements)) = self.arguments.split_last() {
            write!(f, "{}(", self.name)?;
            for arg in elements {
                arg.fmt(f)?;
                write!(f, ",")?;
            }
            last.fmt(f)?;
            write!(f, ")")
        } else {
            write!(f, "{}", self.name)
        }
    }
}

impl Term {
    // TODO: Should parse "f(g(a, b))" to `Symbol { "f", [ Symbol { "g", ["a", "b"] } ] }`
    #[allow(dead_code)]
    fn from(_raw: String) -> Term {
        unimplemented!()
    }

    pub fn var(name: &str) -> Term {
        Term {
            name: name.to_string(),
            arguments: vec![],
        }
    }

    pub fn fun(name: &str, args: Vec<Term>) -> Term {
        Term {
            name: name.to_string(),
            arguments: args.into_iter().map(|arg| Box::new(arg)).collect(),
        }
    }

    pub fn subterms(&self) -> SubTerms {
        SubTerms { stack: vec![self] }
    }
}

pub struct SubTerms<'a> {
    stack: Vec<&'a Term>,
}

impl<'a> Iterator for SubTerms<'a> {
    type Item = &'a Term;

    fn next(&mut self) -> Option<&'a Term> {
        if self.stack.len() == 0 {
            None
        } else {
            let cur: Option<&Term> = self.stack.pop();
            for term in cur.iter() {
                for t in term.arguments.iter() {
                    self.stack.push(&**t)
                }
            }
            cur
        }
    }
}

#[derive(Debug)]
pub enum Kind {
    Equal,
    NotEqual,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Equal => write!(f, "="),
            Kind::NotEqual => write!(f, "!="),
        }
    }
}

/// Equality relation
#[derive(Debug)]
pub struct Relation {
    pub left: Term,
    pub right: Term,
    pub kind: Kind,
}

impl Display for Relation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.kind, self.right)
    }
}

impl Relation {
    pub fn subterms(&self) -> Chain<SubTerms<'_>, SubTerms<'_>> {
        self.left.subterms().chain(self.right.subterms())
    }

    #[allow(dead_code)]
    pub fn parse(raw: String) -> Relation {
        let mut kind: Kind = Kind::Equal;
        let mut eq_index = 0;

        for char in raw.chars().enumerate() {
            match char {
                (i, '=') => {
                    kind = Kind::Equal;
                    eq_index = i
                }
                (i, '!') if raw.chars().nth(i + 1).expect("Invalid symbol") == '=' => {
                    kind = Kind::NotEqual;
                    eq_index = i;
                }
                _ => {}
            }
        }

        let left_raw = &raw[eq_index..];
        let right_raw = &raw[..eq_index];

        Relation {
            left: Term::from(left_raw.to_string()),
            right: Term::from(right_raw.to_string()),
            kind,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct NodeWrapper<'a>(&'a Term);

pub struct Solver<'a> {
    equal_relations: Vec<&'a Relation>,
    not_equal_relations: Vec<&'a Relation>,
    subterms: Vec<&'a Term>,
    dag: Dag<NodeWrapper<'a>, ()>,
    union_find: UnionFind,
}

impl<'a> Solver<'a> {
    pub fn from(relations: &'a Vec<Relation>) -> Solver<'a> {
        let (equal_relations, not_equal_relations) = Solver::split_relations(relations);
        let subterms = Solver::compute_subterms(&relations);
        let dag = Solver::compute_dag(&subterms);
        let union_find = Solver::compute_union_find(&equal_relations, &subterms);

        Solver {
            equal_relations,
            not_equal_relations,
            subterms,
            dag,
            union_find,
        }
    }

    fn split_relations(relations: &'a Vec<Relation>) -> (Vec<&Relation>, Vec<&Relation>) {
        let equal_relations = relations
            .iter()
            .filter(|relation| match relation.kind {
                Kind::Equal => true,
                Kind::NotEqual => false,
            })
            .collect::<Vec<_>>();

        let not_equal_relations = relations
            .iter()
            .filter(|relation| match relation.kind {
                Kind::Equal => false,
                Kind::NotEqual => true,
            })
            .collect::<Vec<_>>();

        (equal_relations, not_equal_relations)
    }

    fn compute_subterms(relations: &Vec<Relation>) -> Vec<&Term> {
        let mut all_subterms: Vec<&Term> = vec![];

        let mut visited: HashSet<&Term> = HashSet::new();

        for relation in relations.iter() {
            println!("Relation: {}", relation);
            let subterms = relation.subterms().collect::<HashSet<_>>();
            for subterm in subterms {
                if !visited.contains(subterm) {
                    all_subterms.push(subterm);
                    visited.insert(subterm);
                }
            }
        }
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

    fn compute_union_find(equal_relations: &Vec<&Relation>, subterms: &Vec<&Term>) -> UnionFind<usize> {
        let all_subterms_indices: HashMap<&Term, usize> = subterms
            .iter()
            .enumerate()
            .map(|(i, term)| (*term, i))
            .collect();

        let mut union_find = UnionFind::new(subterms.len());

        for relation in equal_relations {
            let left_index = all_subterms_indices[&relation.left];
            let right_index = all_subterms_indices[&relation.right];
            union_find.union(right_index, left_index);
        }

        println!("Union-Find");
        for subterm in subterms.iter() {
            let index = all_subterms_indices[subterm];
            println!(
                "{}: {} -> class #{}",
                index,
                subterm,
                union_find.find(index)
            );
        }
        println!();

        union_find
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
                    result.push(format!("{} ~ {}", self.subterms[index1], self.subterms[index2]));
                }
            }
        }
        result
    }

    fn all_congruent_predecessors(&self, node: NodeIndex) -> Vec<NodeIndex> {
        self.dag
            .parents(node)
            .iter(&self.dag)
            .filter(|(_, pred)| self.congruent(node, *pred))
            .map(|(_, pred)| pred)
            .collect()
    }

    fn merge(&mut self, node1: NodeIndex, node2: NodeIndex) {
        let index1 = node1.index();
        let index2 = node2.index();
        let node1_class = self.union_find.find(index1);
        let node2_class = self.union_find.find(index2);

        if node1_class != node2_class {
            let preds1 = self.all_congruent_predecessors(node1);
            let preds2 = self.all_congruent_predecessors(node2);

            self.union_find.union(index1, index2);

            for (pred1, pred2) in preds1.iter().zip(preds2.iter()) {
                let index1 = pred1.index();
                let index2 = pred2.index();
                let pred1_class = self.union_find.find(index1);
                let pred2_class = self.union_find.find(index2);
                if pred1_class != pred2_class && self.congruent(*pred1, *pred2) {
                    println!("Congruence found: {} ~ {}", self.subterms[index1], self.subterms[index2]);
                    self.merge(*pred1, *pred2);
                }
            }
        }
    }

    pub fn check_satisfiable(&mut self) -> bool {
        let equal_relations = self.equal_relations.clone();
        let not_equal_relations = self.not_equal_relations.clone();

        for relation in equal_relations {
            let index1 = self.subterms.iter().position(|term| *term == &relation.left).unwrap();
            let index2 = self.subterms.iter().position(|term| *term == &relation.right).unwrap();

            let dag_index1 = self.dag.from_index(index1);
            let dag_index2 = self.dag.from_index(index2);
            self.merge(dag_index1, dag_index2)
        }

        for relation in not_equal_relations {
            let index1 = self.subterms.iter().position(|term| *term == &relation.left).unwrap();
            let index2 = self.subterms.iter().position(|term| *term == &relation.right).unwrap();

            let class1 = self.union_find.find(index1);
            let class2 = self.union_find.find(index2);
            if class1 == class2 {
                return false;
            }
        }
        return true;
    }
}
