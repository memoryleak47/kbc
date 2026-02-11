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

use std::fmt::*;

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}: {})", self.sym, self.size)
    }
}

impl Debug for Entry { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self}") } }
