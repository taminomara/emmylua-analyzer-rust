mod compilation;
mod diagnostic;
mod vfs;

#[allow(unused)]
pub use compilation::*;
#[allow(unused)]
pub use diagnostic::*;
#[allow(unused)]
pub use vfs::*;

pub struct EmmyLuaAnalysis {}

impl EmmyLuaAnalysis {
    pub fn new() -> Self {
        Self {}
    }
}
