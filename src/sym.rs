use std::fmt::*;
use std::sync::*;

static SYMBOL_MAP: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::from(Vec::new()));

pub fn init_symbol_map(map: Vec<String>) {
    let mut g = SYMBOL_MAP.lock().unwrap();
    assert!(g.is_empty());
    *g = map;
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct Sym {
    // positive or zero: function
    // negative: variable
    repr: i32,
}

impl Sym {
    pub fn is_fn(&self) -> bool { self.repr >= 0 }
    pub fn is_var(&self) -> bool { self.repr < 0 }
}

impl Display for Sym {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let i = self.repr;
        if i >= 0 {
            let i = i as usize;
            let g = SYMBOL_MAP.lock().unwrap();
            let s = &g[i];
            write!(f, "{s}")
        } else {
            let i = -i;
            write!(f, "X{i}")
        }
    }
}

impl Debug for Sym { fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{self}") } }
