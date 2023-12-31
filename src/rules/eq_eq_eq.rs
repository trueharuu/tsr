use swc_common::{errors::Applicability, Span, Spanned};
use swc_ecma_ast::BinaryOp;
use swc_ecma_visit::Visit;
use yansi::Color;

use crate::{
    common::traits::Painted,
    ctx::Ctx,
    lint::{Lint, Metadata},
};

pub struct EqEqEq<'a>(Ctx<'a>);
impl<'a> Metadata<'a> for EqEqEq<'a> {
    fn name(&self) -> &'static str {
        "eq_eq_eq"
    }

    fn new(ctx: Ctx<'a>) -> Self
    where
        Self: Sized,
    {
        Self(ctx)
    }

    fn ctx(&self) -> &Ctx {
        &self.0
    }
}

impl Lint<'_> for EqEqEq<'_> {}

impl Visit for EqEqEq<'_> {
    fn visit_bin_expr(&mut self, n: &swc_ecma_ast::BinExpr) {
        if let BinaryOp::EqEq | BinaryOp::NotEq = n.op {
            self.report(n.span, "use of non-strict equality operator", |f, s| {
                let m = self
                    .ctx()
                    .lexer()
                    .clone()
                    .find(|x| n.left.span().between(n.right.span()).contains(x.span()))
                    .map(|x| x.span)
                    .unwrap();
                f.set_span(m);
                f.span_suggestion_with_applicability(
                    s,
                    "non-strict equality can give unwanted results",
                    format!(
                        "use '{}' instead",
                        if n.op == BinaryOp::EqEq {
                            BinaryOp::EqEqEq
                        } else {
                            BinaryOp::NotEqEq
                        }
                    ),
                    Applicability::MaybeIncorrect,
                );
            });
        }
    }
}
