use swc_ecma_visit::{Fold, Visit};

use crate::lint::Lint;

pub struct EqEqEq;
impl Lint for EqEqEq {
    fn name(&self) -> &'static str {
        "eq_eq_eq"
    }
}
impl Visit for EqEqEq {
    fn visit_bin_expr(&mut self, n: &swc_ecma_ast::BinExpr) {
        println!("!")
    }
}

impl Fold for EqEqEq {}
