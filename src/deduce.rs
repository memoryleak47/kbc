use crate::*;

// assumes variables are disjoint.
pub fn deduce_one(x: &Equation, y: &Equation, p: usize) -> Option<Equation> {
    assert!(p < y.lhs.len());

    let sig = unify(&x.lhs, pos_idx(&y.lhs, p))?;
    let sub = pos_set(&y.lhs, p, &x.rhs);
    let ll = apply_subst(&sub, &sig);
    let rr = apply_subst(&y.rhs, &sig);
    let eq = Equation { lhs: ll, rhs: rr, oriented: false };
    Some(eq)
}

// t[p]
fn pos_idx(t: &FlatTerm, p: usize) -> &FlatTerm {
    &t[p..]
}

// t[p := t2]
fn pos_set(t: &FlatTerm, p: usize, t2: &FlatTerm) -> Box<FlatTerm> {
    todo!()
}
