use crate::*;

pub fn make_odd(t: &mut FlatTerm) {
    for Entry { sym, .. } in t {
        if sym.is_var() {
            sym.repr = sym.repr * 2 + 1;
        }
    }
}

pub fn make_even(t: &mut FlatTerm) {
    for Entry { sym, .. } in t {
        if sym.is_var() {
            sym.repr = sym.repr * 2;
        }
    }
}

pub fn rename_canon(e: &mut Equation) {
    let mut v: Vec<Sym> = Vec::new();
    for Entry { sym: x, .. } in e.lhs.iter_mut().chain(e.rhs.iter_mut()) {
        if x.is_var() {
            if !v.contains(&x) { v.push(*x); }

            let p = v.iter().position(|a| a == x).unwrap();
            *x = Sym::mk_var(p as _);
        }
    }
}

pub fn rename_canon_term(t: &mut FlatTerm) {
    let mut v: Vec<Sym> = Vec::new();
    for Entry { sym: x, .. } in t.iter_mut() {
        if x.is_var() {
            if !v.contains(&x) { v.push(*x); }

            let p = v.iter().position(|a| a == x).unwrap();
            *x = Sym::mk_var(p as _);
        }
    }
}

pub fn is_canon_term(t: &FlatTerm) -> bool {
    let mut tt = ft_box(t);
    rename_canon_term(&mut *tt);
    &*tt == t
}
