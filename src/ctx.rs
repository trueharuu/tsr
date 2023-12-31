
use swc_common::sync::Lrc;
use swc_common::{errors::Handler, SourceFile, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser};

use crate::levels::{Level, Levels};

pub struct Ctx<'a> {
    pub cm: Lrc<SourceMap>,
    pub h: Handler,
    pub fm: &'a Lrc<SourceFile>,
    pub lx: Lexer<'a>,
    pub pr: Parser<Lexer<'a>>,
    pub lv: Levels,
}

impl<'a> Ctx<'a> {
    pub fn handler(&self) -> &Handler {
        &self.h
    }

    pub fn source_map(&self) -> &SourceMap {
        self.cm.as_ref()
    }

    pub fn file(&self) -> &SourceFile {
        self.fm.as_ref()
    }

    pub fn lexer(&self) -> &Lexer<'a> {
        &self.lx
    }

    pub fn parser(&self) -> &Parser<Lexer<'a>> {
        &self.pr
    }

    pub fn levels(&self) -> &Levels {
        &self.lv
    }

    pub fn level_of(&self, label: &str) -> Option<&Level> {
        self.levels().get(label)
    }
}
