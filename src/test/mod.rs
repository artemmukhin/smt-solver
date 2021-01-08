#[cfg(test)]
mod tests {
    use crate::{Relation, Term, Kind};

    #[test]
    fn test_subterms_simple_vars() {
        // `x = y`
        test_subterms(
            Relation {
                left: Term::var("x"),
                right: Term::var("y"),
                kind: Kind::Equal,
            }, vec![
                "x",
                "y",
            ],
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
            }, vec![
                "f(x)",
                "x",
                "g(x)",
                "x",
            ],
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
            }, vec![
                "f(x,y)",
                "y",
                "x",
                "g(x)",
                "x",
            ],
        );
    }

    #[test]
    fn test_subterms_2() {
        // `u(v(a), t(b, a)) != a`
        test_subterms(
            Relation {
                left: Term::fun("u", vec![
                    Term::fun("v", vec![Term::var("a")]),
                    Term::fun("t", vec![Term::var("b"), Term::var("a")]),
                ]),
                right: Term::var("a"),
                kind: Kind::NotEqual,
            }, vec![
                "u(v(a),t(b,a))",
                "t(b,a)",
                "a",
                "b",
                "v(a)",
                "a",
                "a",
            ],
        );
    }

    fn test_subterms(relation: Relation, expected: Vec<&str>) {
        let subterms = relation
            .sub_terms()
            .map(|term| format!("{}", term))
            .collect::<Vec<String>>();

        assert_eq!(&subterms[..], &expected[..]);
    }
}
