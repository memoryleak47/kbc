mod minqueue;
pub use minqueue::*;

mod flatterm;
pub use flatterm::*;

use std::collections::HashMap;

type DiscrMap = HashMap<Sym, DiscrNode>;

enum DiscrNode {
    Branch(DiscrMap),
    Leaf(Vec<EqId>),
}

pub struct Equation {
    lhs: Box<FlatTerm>,
    rhs: Box<FlatTerm>,
    oriented: bool,
}



type RuleId = usize;
type Pos = usize;
type Score = usize;

struct Passive {
    lhs: RuleId,
    rhs: RuleId,
    pos: Pos,
}

pub type EqId = usize; // index into equations.

pub struct State {
    discr: DiscrMap,
    equations: Vec<Equation>,
    passive: MinPrioQueue<Score, Passive>,
}

fn main() {
}
