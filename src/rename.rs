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
