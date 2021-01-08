use smt_solver;
use smt_solver::{Relation, Term, Kind};
use std::collections::HashSet;

fn get_unsat_example() -> Vec<Relation> {
    vec![
        // `f(x, y) = x`
        Relation {
            left: Term::fun("f", vec![Term::var("x"), Term::var("y")]),
            right: Term::fun("g", vec![Term::var("x")]),
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

fn print_relation(relation: Relation) {
    println!("Relation: {}", relation);
    let subterms = relation.subterms().collect::<HashSet<_>>();
    println!("Subterms:");
    for subterm in subterms {
        println!("{}", subterm);
    }
    println!();
}

fn main() {
    let relations = get_unsat_example();
    for relation in relations {
        print_relation(relation);
    }
}
