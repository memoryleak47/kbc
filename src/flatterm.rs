use crate::*;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Entry {
    pub sym: Sym,
    pub size: u32, // size of that subterm
}

pub type FlatTerm = [Entry];

pub fn ft_child(t: &FlatTerm) -> &FlatTerm {
    &t[1..]
}

pub fn ft_next(t: &FlatTerm) -> &FlatTerm {
    let Entry { size, .. } = t[0];
    let size = size as usize;
    &t[size..]
}

pub fn ft_children(t: &FlatTerm) -> impl Iterator<Item=&FlatTerm> {
    let mut i = 1;
    let size = t[0].size as usize;
    std::iter::from_fn(move || {
        if i >= size { return None }
        let e = t[i];
        let current_i = i;
        i += e.size as usize;
        Some(&t[current_i..])
    })
}

use std::fmt::*;

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}: {})", self.sym, self.size)
    }
}

impl Debug for Entry { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self}") } }
