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

    let sub_terms: Vec<_> = relation.sub_terms().collect();
    println!("Subterms");
    for sub_term in sub_terms {
        println!("{}", sub_term);
    }
}
