use std::cell::OnceCell;
use std::sync::OnceLock;
use crate::sys::*;

mod raw;
use raw::*;
use crate::{BinaryOp, Module, Type};

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ExpressionId(BinaryenExpressionId);

pub trait Expression {
    fn id() -> ExpressionId where Self: Sized;
    fn as_raw(&self) -> &RawExpression;
    fn as_raw_mut(&mut self) -> &mut RawExpression;
}

macro_rules! expression {
    ($name:ident, $idfunc:ident) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Hash)]
        pub struct $name(RawExpression);

        impl Expression for $name {
            #[inline]
            fn id() -> ExpressionId {
                unsafe { ExpressionId($idfunc()) }
            }

            #[inline]
            fn as_raw(&self) -> &RawExpression {
                &self.0
            }

            #[inline]
            fn as_raw_mut(&mut self) -> &mut RawExpression {
                &mut self.0
            }
        }
    }
}

expression!(Block, BinaryenBlockId);

expression!(Nop, BinaryenNopId);

impl Nop {
    pub fn new(module: &Module) -> &mut Nop {
        unsafe {
            &mut *(BinaryenNop(module.as_ptr()).cast())
        }
    }
}

expression!(Binary, BinaryenBinaryId);

impl Binary {
    pub fn new<'ctx>(module: &'ctx Module, op: BinaryOp, left: &'ctx dyn Expression, right: &'ctx dyn Expression) -> &'ctx mut Binary {
        unsafe {
            &mut *(BinaryenBinary(module.as_ptr(), op.into(), left.as_raw().as_ptr(), right.as_raw().as_ptr())).cast()
        }
    }
}

expression!(LocalGet, BinaryenLocalGetId);

impl LocalGet {
    pub fn new(module: &Module, index: u32, type_: Type) -> &mut LocalGet {
        unsafe {
            &mut *(BinaryenLocalGet(module.as_ptr(), index, type_.to_raw()).cast())
        }
    }
}
