#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Sym {
    // positive: function
    // negative: variable
    repr: i32,
}

impl Sym {
    pub fn is_var(&self) -> bool { self.repr < 0 }
    pub fn is_fn(&self) -> bool { self.repr > 0 }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Entry {
    pub sym: Sym,
    pub size: u32, // size of that subterm
}

pub type FlatTerm = [Entry];
