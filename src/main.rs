use smt_solver;
use smt_solver::{Relation, Term, Kind};
use std::collections::{HashSet, HashMap};
use disjoint_sets::UnionFind;
use daggy::{Dag, NodeIndex};

fn get_unsat_example() -> Vec<Relation> {
    vec![
        // `f(x, y) = x`
        Relation {
            left: Term::fun("f", vec![Term::var("x"), Term::var("y")]),
            right: Term::var("x"),
            kind: Kind::Equal,
        },
        // `f(f(x, y), y) != x`
        Relation {
            left: Term::fun("f", vec![
                Term::fun("f", vec![Term::var("x"), Term::var("y")]),
                Term::var("y")
            ]),
            right: Term::var("x"),
            kind: Kind::NotEqual,
        },
    ]
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Node<'a>(&'a Term);

fn main() {
    let relations = get_unsat_example();
    let mut all_subterms: HashMap<Term, usize> = HashMap::new();
    let mut index = 0;

    for relation in relations.iter() {
        println!("Relation: {}", relation);
        let subterms = relation.subterms().collect::<HashSet<_>>();
        for subterm in subterms {
            if !all_subterms.contains_key(subterm) {
                all_subterms.insert(subterm.clone(), index);
                index += 1;
            }
        }
    }
    println!();

    println!("Subterms:");
    for subterm in all_subterms.keys() {
        println!("{}", subterm);
    }
    println!();

    let mut union_find = UnionFind::new(all_subterms.len());
    let eq_relations = relations
        .iter()
        .filter(|relation| match relation.kind {
            Kind::Equal => true,
            Kind::NotEqual => false
        })
        .collect::<Vec<_>>();

    for relation in eq_relations {
        let s = *all_subterms.get(&relation.left).unwrap();
        let t = *all_subterms.get(&relation.right).unwrap();
        union_find.union(s, t);
    }
    
    let mut dag = Dag::<Node, (), u32>::new();

    let mut nodes: HashMap<&Term, NodeIndex> = HashMap::new();
    
    for (subterm, _) in all_subterms.iter() {
        let node = Node(&subterm);
        let index = dag.add_node(node);
        nodes.insert(&subterm, index);
    }
    for (subterm, _) in all_subterms.iter() {
        let node = nodes[&subterm];
        let children = subterm.arguments
            .iter()
            .map(|arg| nodes[&**arg])
            .collect::<Vec<_>>();
        for child in children {
            dag.add_edge(node, child, ());
        }
    }

    // TODO: propagate the new congruence with symmetry, transitivity, and functional congruence

    println!("Union-Find");
    for (subterm, i) in all_subterms.iter() {
        println!("{} -> {}", subterm, union_find.find(*i));
    }
}
