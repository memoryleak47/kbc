use crate::*;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Entry {
    pub sym: Sym,
    pub size: u32, // size of that subterm
}

// contains only a single term, so t[0].size == t.len()
pub type FlatTerm = [Entry];

// might contain multiple terms.
pub type FlatTermList = [Entry];

pub fn ft_child(t: &FlatTerm) -> &FlatTerm {
    ft_shrink(&t[1..])
}

pub fn ft_check(t: &FlatTerm) {
    let n = t.len();

    let mut s = 1;
    for a in ft_children(t) {
        ft_check(a);
        s += a[0].size as usize;
    }
    assert!(n == t[0].size as usize);
    assert!(n == s);
}

// reduces a term list to its first element.
pub fn ft_shrink(t: &FlatTermList) -> &FlatTerm {
    let size = t[0].size as usize;
    &t[0..size]
}

pub fn ft_next(t: &FlatTermList) -> &FlatTermList {
    let Entry { size, .. } = t[0];
    let size = size as usize;
    &t[size..]
}

pub fn ft_box(t: &FlatTerm) -> Box<FlatTerm> {
    t.iter().copied().collect()
}


pub fn ft_children(t: &FlatTerm) -> impl Iterator<Item=&FlatTerm> {
    let mut i = 1;
    let size = t[0].size as usize;
    std::iter::from_fn(move || {
        if i >= size { return None }
        let e = t[i];
        let current_i = i;
        let esize = e.size as usize;
        i += esize;
        Some(&t[current_i..(current_i+esize)])
    })
}

pub fn ft_string(t: &FlatTerm) -> String {
    let Entry { sym, size } = t[0];
    let mut s = sym.to_string();
    if size == 1 {
        return s
    }
    s.push('(');
    for c in ft_children(t) {
        s.push_str(&ft_string(c));
        s.push_str(", ");
    }
    s.pop().unwrap();
    s.pop().unwrap();
    s.push(')');
    s
}

pub fn is_ground(t: &FlatTerm) -> bool {
    for Entry { sym, .. } in t {
        if sym.is_var() { return false }
    }
    true
}
