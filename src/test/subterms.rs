#![cfg(test)]

use crate::{Kind, Relation, Term};
use std::collections::HashSet;

#[test]
fn test_subterms_simple_vars() {
    // `x = y`
    test_subterms(
        Relation {
            left: Term::var("x"),
            right: Term::var("y"),
            kind: Kind::Equal,
        },
        vec!["x", "y"],
    );
}

#[test]
fn test_subterms_simple_funcs() {
    // `f(x) = g(x)`
    test_subterms(
        Relation {
            left: Term::fun("f", vec![Term::var("x")]),
            right: Term::fun("g", vec![Term::var("x")]),
            kind: Kind::Equal,
        },
        vec!["x", "f(x)", "g(x)"],
    );
}

#[test]
fn test_subterms_1() {
    // `f(x, y) = g(x)`
    test_subterms(
        Relation {
            left: Term::fun("f", vec![Term::var("x"), Term::var("y")]),
            right: Term::fun("g", vec![Term::var("x")]),
            kind: Kind::Equal,
        },
        vec!["x", "y", "f(x,y)", "g(x)"],
    );
}

#[test]
fn test_subterms_2() {
    // `u(v(a), t(b, a)) != a`
    test_subterms(
        Relation {
            left: Term::fun(
                "u",
                vec![
                    Term::fun("v", vec![Term::var("a")]),
                    Term::fun("t", vec![Term::var("b"), Term::var("a")]),
                ],
            ),
            right: Term::var("a"),
            kind: Kind::NotEqual,
        },
        vec!["a", "b", "v(a)", "t(b,a)", "u(v(a),t(b,a))"],
    );
}

fn test_subterms(relation: Relation, expected: Vec<&str>) {
    let subterms_set = relation
        .subterms()
        .map(|term| format!("{}", term))
        .collect::<HashSet<_>>();
    let expected_set = expected
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    assert_eq!(subterms_set, expected_set);
}
