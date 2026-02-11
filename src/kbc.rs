use crate::*;

pub fn kbc(eqs: Vec<Equation>) {
    let mut st = State::new();
    for e in eqs {
        st.enqueue(e);
    }

    while let Some((_, x)) = st.passive.pop() {
        st.add_active(x);
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            passive: MinPrioQueue::new(),
            active: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, eq: Equation) {
        let score = eq.lhs[0].size + eq.rhs[0].size;
        self.passive.push(score, eq);
    }

    pub fn add_active(&mut self, mut e: Equation) {
        if e.lhs == e.rhs { return }
        make_canon(&mut e);
        if self.active.contains(&e) { return }

        {
            let mut e2 = e.clone();
            make_odd(&mut e2.lhs);
            make_odd(&mut e2.rhs);

            let mut cps = Vec::new();
            for mut a in self.active.iter().cloned() {
                make_even(&mut a.lhs);
                make_even(&mut a.rhs);
                deduce(&a, &e2, &mut cps);
            }
            for cp in cps {
                self.enqueue(cp);
            }
        }

        let i = self.active.len();
        println!("{i}: {:?} = {:?}", e.lhs, e.rhs);
        self.active.push(e);
    }
}

fn make_canon(e: &mut Equation) {
    if gt(&e.lhs, &e.rhs) {
        e.oriented = true;
        rename_canon(e);
        return
    }

    if gt(&e.rhs, &e.lhs) {
        std::mem::swap(&mut e.lhs, &mut e.rhs);
        e.oriented = true;
        rename_canon(e);
        return
    }

    e.oriented = false;

    let mut e2 = e.clone();
    std::mem::swap(&mut e2.lhs, &mut e2.rhs);

    rename_canon(e);
    rename_canon(&mut e2);

    if &e2 < e { std::mem::swap(e, &mut e2); }
}
