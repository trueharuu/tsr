#![feature(let_chains, specialization)] // wow
pub mod common;
pub mod lint;
pub mod program;
pub mod rules;
use std::path::Path;

use lint::Lint;
use rules::lints;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::sync::Lrc;
use swc_common::SourceMap;
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, TsConfig};
use swc_ecma_visit::FoldWith;

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

    let mut pr = Parser::new_from(lx);
    for e in pr.take_errors() {
        e.into_diagnostic(&h).emit();
    }

    let m = pr
        .parse_module()
        .map_err(|e| e.into_diagnostic(&h).emit())
        .unwrap();

    for mut i in lints().into_iter() {
        m.body.clone().fold_children_with(&mut *i);
    }
}
