use crate::lint::Lint;

pub mod eq_eq_eq;

pub fn lints() -> Vec<Box<dyn Lint>> {
    vec![Box::new(eq_eq_eq::EqEqEq)]
}
