use crate::*;

pub fn unify(l: &FlatTerm, r: &FlatTerm) -> Option<Subst> {
    ft_check(l);
    ft_check(r);

    let mut subst = Default::default();
    unify_impl(l, r, &mut subst)?;
    Some(subst)
}

fn box_ft(ft: &FlatTerm) -> Box<FlatTerm> {
    ft.iter().copied().collect()
}

// invariants: subst is always fully simplified w.r.t. itself.
fn unify_impl(l: &FlatTerm, r: &FlatTerm, subst: &mut Subst) -> Option<()> {
    if l == r { return Some(()) }

    // replace defined vars.
    let lv = l[0].sym;
    if lv.is_var() && let Some(lt) = subst.get(&lv) {
        return unify_impl(&lt.clone(), r, subst);
    }
    let rv = r[0].sym;
    if rv.is_var() && let Some(rt) = subst.get(&rv) {
        return unify_impl(l, &rt.clone(), subst);
    }

    // define vars.
    if lv.is_var() && subst.get(&lv).is_none() {
        subst_add(lv, box_ft(r), subst)?;
        return unify_impl(l, r, subst);
    }
    if rv.is_var() && subst.get(&rv).is_none() {
        subst_add(rv, box_ft(l), subst)?;
        return unify_impl(l, r, subst);
    }

    assert!(lv.is_fn());
    assert!(rv.is_fn());

    if lv != rv { return None }
    for (ll, rr) in ft_children(l).zip(ft_children(r)) {
        unify_impl(ll, rr, subst)?;
    }
    Some(())
}

fn subst_add(v: Sym, t: Box<FlatTerm>, subst: &mut Subst) -> Option<()> {
    ft_check(&t);
    let t = fix_apply_subst(t, subst);

    // nothing to be added.
    if t[0].sym == v { return Some(()) }

    // cyclic definition, forbidden!
    if contains_var(&t, v) { return None }

    subst.insert(v, t);

    let old_subst = subst.clone();

    for (_, tt) in subst.iter_mut() {
        *tt = fix_apply_subst(tt.clone(), &old_subst);
    }

    Some(())
}

fn fix_apply_subst(mut t: Box<FlatTerm>, subst: &Subst) -> Box<FlatTerm> {
    loop {
        let t2 = apply_subst(&t, subst);
        if t == t2 { return t }
        t = t2;
    }
}

fn contains_var(t: &FlatTerm, v: Sym) -> bool {
    for Entry { sym, .. } in t {
        if *sym == v { return true }
    }
    false
}
