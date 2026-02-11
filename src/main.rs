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

mod state;
pub use state::*;

mod kbc;
pub use kbc::*;

mod rename;
pub use rename::*;

mod unify;
pub use unify::*;

mod deduce;
pub use deduce::*;

pub use std::collections::{HashMap, BTreeMap};

fn main() {
    let s = std::fs::read_to_string("example.p").unwrap();
    let eqs = parse_file(&s);
    kbc(eqs);
}
