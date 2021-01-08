use std::collections::HashSet;

use smt_solver;
use smt_solver::{Kind, Relation, Solver, Term};

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
            left: Term::fun(
                "f",
                vec![
                    Term::fun("f", vec![Term::var("x"), Term::var("y")]),
                    Term::var("y"),
                ],
            ),
            right: Term::var("x"),
            kind: Kind::NotEqual,
        },
    ]
}

fn main() {
    let relations = get_unsat_example();
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
    let all_subterms = all_subterms;

    println!("Subterms:");
    for subterm in all_subterms.iter() {
        println!("{}", subterm);
    }
    println!();

    let solver = Solver::from(&all_subterms, &relations);
    let congruent = solver.find_congruent();
    
    println!("Congruent:");
    for (index1, index2) in congruent {
        println!("{} ~ {}", all_subterms[index1], all_subterms[index2]);
    }
}
