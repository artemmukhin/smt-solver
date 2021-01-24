use smt_solver::solver::Solver;
use smt_solver::{Kind, Relation, Term};

fn main() {
    let relations = vec![
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
    ];

    let mut solver = Solver::from(relations.iter().collect());
    let is_satisfiable = solver.check_satisfiable();
    if is_satisfiable {
        println!("Satisfiable");
    } else {
        println!("Unsatisfiable");
    }
}
