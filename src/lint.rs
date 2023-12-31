use std::fmt::Display;

use swc_common::{errors::DiagnosticBuilder, Span};
use swc_ecma_visit::Visit;
use yansi::Color;

use crate::{common::traits::Painted, ctx::Ctx, levels::Level};

pub trait Metadata<'a>: Visit {
    fn name(&self) -> &'static str;

    fn new(ctx: Ctx<'a>) -> Self
    where
        Self: Sized;
    fn ctx(&self) -> &Ctx;
}

pub trait Lint<'a>: Visit {
    fn report<F>(&self, on: Span, msg: impl Display, f: F)
    where
        Self: Metadata<'a>,
        F: Fn(&mut DiagnosticBuilder, Span),
    {
        let lv = self.ctx().level_of(self.name()).unwrap_or(&Level::Allow);

        let mut swc_lv = match lv {
            Level::Allow => return,
            Level::Warn => self.ctx().handler().struct_span_warn(on, &msg.to_string()),
            Level::Deny => self.ctx().handler().struct_span_warn(on, &msg.to_string()),
        };

        f(&mut swc_lv, on);
        swc_lv.note(&format!(
            "error occurs because lint {} is enabled",
            format!("`{}`", self.name()).painted().fg(Color::Blue)
        ));

        swc_lv.emit();
    }
}
