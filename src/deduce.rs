use crate::*;

// assumes variables are disjoint.
pub fn deduce_one(l: &Equation, r: &Equation, p: usize) -> Option<Equation> {
    assert!(p < r.lhs.len());

    let Some(sig) = unify(&l.lhs, pos_idx(&r.lhs, p)) else { return None };
    let sub = pos_set(&r.lhs, p, &l.rhs);
    let ll = apply_subst(&sub, &sig);
    let rr = apply_subst(&r.rhs, &sig);
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
