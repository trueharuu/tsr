#![feature(let_chains)] // wow
pub mod common;
pub mod ctx;
pub mod levels;
pub mod lint;
pub mod program;
pub mod rules;
use std::path::Path;

use ctx::Ctx;
use levels::default_levels;
use rules::lints;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, TsConfig};
use swc_ecma_visit::VisitWith;

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let h = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.load_file(Path::new("test/test.ts")).unwrap();
    let lx = Lexer::new(
        swc_ecma_parser::Syntax::Typescript(TsConfig::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut pr = Parser::new_from(lx.clone());
    for e in pr.take_errors() {
        e.into_diagnostic(&h).emit();
    }

    let m = pr
        .parse_module()
        .map_err(|e| e.into_diagnostic(&h).emit())
        .unwrap();

    let ctx = Ctx {
        cm,
        h,
        fm: &fm,
        lx,
        pr,
        lv: default_levels(),
    };

    for mut i in lints(ctx).into_iter() {
        m.body.clone().visit_children_with(&mut *i);
    }
}

