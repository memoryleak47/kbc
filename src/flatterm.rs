use crate::*;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Entry {
    pub sym: Sym,
    pub size: u32, // size of that subterm
}

pub type FlatTerm = [Entry];
