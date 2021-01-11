#![cfg(test)]

use crate::{Solver, Relation, Term, Kind};

#[test]
fn test_congruence_1() {
    test_congruence(
        vec![
            // `f(x, y) = x`
            &Relation {
                left: Term::fun("f", vec![Term::var("x"), Term::var("y")]),
                right: Term::var("x"),
                kind: Kind::Equal,
            },
            // `f(f(x, y), y) != x`
            &Relation {
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
        ],
        vec![
            "f(x,y) ~ f(f(x,y),y)"
        ],
    )
}

fn test_congruence(relations: Vec<&Relation>, expected: Vec<&str>) {
    let mut solver = Solver::from(relations);
    let _ = solver.check_satisfiable();
    let congruent = solver.find_all_congruent_terms();
    assert_eq!(congruent, expected);
}
