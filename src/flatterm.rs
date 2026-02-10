use crate::*;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Entry {
    pub sym: Sym,
    pub size: u32, // size of that subterm
}

pub type FlatTerm = [Entry];

use std::fmt::*;

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}: {})", self.sym, self.size)
    }
}

impl Debug for Entry { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self}") } }
