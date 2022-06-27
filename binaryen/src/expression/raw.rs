use std::cell::UnsafeCell;
use std::sync::OnceLock;
use std::ffi::{CStr, CString};
use std::hash::{Hash, Hasher};
use std::io::{stdout, Write};
use std::ptr::{NonNull, null_mut};
use std::marker::PhantomData;
use std::ptr;
use crate::{ExpressionId, Module, Type};
use crate::sys::*;

#[repr(transparent)]
pub struct RawExpression(UnsafeCell<BinaryenExpression>);

impl PartialEq for RawExpression {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for RawExpression {
}

impl Hash for RawExpression {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self, state)
    }
}

impl RawExpression {
    #[inline]
    pub const fn as_ptr(&self) -> *mut BinaryenExpression {
        self.0.get()
    }

    #[inline]
    pub fn clone<'m>(&self, module: &'m Module) -> &'m RawExpression {
        unsafe { &*BinaryenExpressionCopy(self.0.get(), module.as_ptr()).cast() }
    }

    #[inline]
    pub fn id(&self) -> BinaryenExpressionId {
        unsafe { BinaryenExpressionGetId(self.as_ptr()) }
    }

    #[inline]
    pub fn type_(&self) -> BinaryenType {
        unsafe { BinaryenExpressionGetType(self.as_ptr()) }
    }

    #[inline]
    pub fn set_type(&mut self, type_: BinaryenType) {
        unsafe { BinaryenExpressionSetType(self.as_ptr(), type_) }
    }

    #[inline]
    pub fn print(&self) {
        unsafe { BinaryenExpressionPrint(self.as_ptr()) }
    }

    #[inline]
    pub fn finalize(&mut self) {
        unsafe { BinaryenExpressionFinalize(self.as_ptr()) }
    }
}
