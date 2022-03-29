use math::expr::Expr;
use std::collections::HashMap;

pub struct Environment {
    store: HashMap<String, Expr>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, s: String, e: Expr) {
        self.store.insert(s, e);
    }

    pub fn get(&self, s: &str) -> Expr {
        self.store
            .get(s)
            .and_then(|e| Some(e.clone()))
            .unwrap_or_else(|| Expr::Sym(s.to_string()))
    }
}