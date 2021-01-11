#![cfg(test)]

use crate::{Solver, Relation, Term, Kind};

#[test]
fn test_sat_1() {
    test_satisfiability(
        vec![
            // `f(x, y) = x`
            &Relation {
                left: Term::fun("f", vec![Term::var("x"), Term::var("y")]),
                right: Term::var("x"),
                kind: Kind::Equal,
            },
            // `f(f(x, x), x) = x`
            &Relation {
                left: Term::fun(
                    "f",
                    vec![
                        Term::fun("f", vec![Term::var("x"), Term::var("x")]),
                        Term::var("x"),
                    ],
                ),
                right: Term::var("x"),
                kind: Kind::Equal,
            },
        ],
        true,
    )
}

#[test]
fn test_sat_2() {
    test_satisfiability(
        vec![
            // f(f(x)) = x
            &Relation {
                left: Term::fun(
                    "f",
                    vec![Term::fun("f", vec![Term::var("x")])],
                ),
                right: Term::var("x"),
                kind: Kind::Equal,
            },

            // f(x) = y
            &Relation {
                left: Term::fun("f", vec![Term::var("x")]),
                right: Term::var("y"),
                kind: Kind::Equal,
            },

            // x != y
            &Relation {
                left: Term::var("x"),
                right: Term::var("y"),
                kind: Kind::NotEqual,
            }
        ],
        true,
    )
}

#[test]
fn test_unsat_1() {
    test_satisfiability(
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
        false,
    )
}

fn test_satisfiability(relations: Vec<&Relation>, expected: bool) {
    let mut solver = Solver::from(relations);
    let is_satisfiable = solver.check_satisfiable();
    assert_eq!(is_satisfiable, expected);
}
