use crate::math::expr::Expr;
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

    pub fn default() -> Environment {
        let mut env = Environment::new();
        env.set("\\diff", crate::math::diff::CMD_DIFF);
        env
    }

    pub fn set(&mut self, s: &str, e: Expr) {
        self.store.insert(s.to_string(), e);
    }

    pub fn get(&self, s: &str) -> Expr {
        self.store
            .get(s)
            .cloned()
            .unwrap_or_else(|| Expr::Sym(s.to_string()))
    }
}
