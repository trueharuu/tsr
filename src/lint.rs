use swc_ecma_visit::{Fold, Visit};

pub trait Lint: Visit + Fold {
    fn name(&self) -> &'static str;
}
