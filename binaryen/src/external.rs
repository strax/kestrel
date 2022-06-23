use crate::sys::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum ExternalKind {
    Function,
    Table,
    Memory,
    Global,
    Tag
}

impl From<ExternalKind> for BinaryenExternalKind {
    #[inline]
    fn from(x: ExternalKind) -> Self {
        unsafe {
            match x {
                ExternalKind::Function => BinaryenExternalFunction(),
                ExternalKind::Table => BinaryenExternalTable(),
                ExternalKind::Memory => BinaryenExternalMemory(),
                ExternalKind::Global => BinaryenExternalGlobal(),
                ExternalKind::Tag => BinaryenExternalTag()
            }
        }
    }
}