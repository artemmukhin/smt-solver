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
    let solver = Solver::from(&relations);
    let _ = solver.find_congruent();
}
