use smt_solver;
use smt_solver::{Relation, Term, Kind};

fn main() {
    // `f(x, y, z) = g(x, z)`
    let relation = Relation {
        left: Term::fun("f", vec![Term::var("x"), Term::var("y"), Term::var("z")]),
        right: Term::fun("g", vec![Term::var("x"), Term::var("z")]),
        kind: Kind::Equal
    };
    println!("Relation");
    println!("{}\n", relation);

    let subterms: Vec<_> = relation.subterms().collect();
    println!("Subterms");
    for subterm in subterms {
        println!("{}", subterm);
    }
}
