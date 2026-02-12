mod sym;
pub use sym::*;

mod flatterm;
pub use flatterm::*;

mod parse;
pub use parse::*;

mod minqueue;
pub use minqueue::*;

mod order;
pub use order::*;

mod subst;
pub use subst::*;

mod pos;
pub use pos::*;

mod state;
pub use state::*;

mod kbc;
pub use kbc::*;

mod index;
pub use index::*;

mod rename;
pub use rename::*;

mod unify;
pub use unify::*;

mod deduce;
pub use deduce::*;

mod simplify;
pub use simplify::*;

pub use std::collections::{HashMap, BTreeMap};
pub use std::collections::hash_map::Entry as HMEntry;

fn main() {
    let s = std::fs::read_to_string("example.p").unwrap();
    let eqs = parse_file(&s);
    kbc(eqs);
}
