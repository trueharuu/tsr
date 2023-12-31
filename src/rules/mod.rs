use crate::{ctx::Ctx, lint::Metadata};

pub mod eq_eq_eq;

pub fn lints<'a>(ctx: Ctx<'a>) -> Vec<Box<dyn Metadata + 'a>> {
    vec![Box::new(eq_eq_eq::EqEqEq::new(ctx))]
}
