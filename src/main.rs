mod minqueue;
pub use minqueue::*;

mod flatterm;
pub use flatterm::*;

use std::collections::BTreeMap;

type DiscrMap = BTreeMap<Sym, DiscrNode>;

enum DiscrNode {
    Node(Option<Sym> /* None == '*' */, DiscrMap),
    Leaf(Vec<EqId>),
}

pub struct Equation {
    lhs: Box<FlatTerm>,
    rhs: Box<FlatTerm>,
    oriented: bool,
}


pub type EqId = usize; // index into equations.
pub struct DiscrTree {
    map: DiscrMap,
    equations: Vec<Equation>,
}

type RuleId = usize;
type Pos = usize;
type Score = usize;

struct Passive {
    lhs: RuleId,
    rhs: RuleId,
    pos: Pos,
}

pub struct State {
    discr: DiscrTree,
    passive: MinPrioQueue<Score, Passive>,
}

fn main() {
}
