#[derive(Clone, Copy)]
pub struct Sym {
    // positive: function
    // negative: variable
    sym: i32,
}

pub struct Entry {
    sym: Sym,
    size: u32, // size of that subterm
}

pub type FlatTerm = [Entry];
