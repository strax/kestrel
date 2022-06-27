use crate::sys::*;
use std::cell::UnsafeCell;
use std::ffi::CStr;
use crate::{Expression, Module, Type};

pub struct FunctionDescriptor<'ctx, 'a> {
    pub name: &'a CStr,
    pub params: Type,
    pub results: Type,
    pub vars: &'a [Type],
    pub body: &'ctx dyn Expression
}

#[repr(transparent)]
pub struct Function(UnsafeCell<BinaryenFunction>);

impl Function {
    #[inline]
    const fn as_ptr(&self) -> *mut BinaryenFunction {
        self.0.get()
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(BinaryenFunctionGetName(self.as_ptr())) }
    }

    #[inline]
    pub fn params(&self) -> Type {
        unsafe { Type::from_raw(BinaryenFunctionGetParams(self.as_ptr())) }
    }

    #[inline]
    pub fn results(&self) -> Type {
        unsafe { Type::from_raw(BinaryenFunctionGetResults(self.as_ptr())) }
    }

    #[inline]
    pub fn num_vars(&self) -> u32 {
        unsafe { BinaryenFunctionGetNumVars(self.as_ptr()) }
    }

    pub fn var(&self, index: u32) -> Type {
        unsafe { Type::from_raw(BinaryenFunctionGetVar(self.as_ptr(), index)) }
    }

    #[inline]
    pub fn num_locals(&self) -> u32 {
        unsafe { BinaryenFunctionGetNumLocals(self.as_ptr()) }
    }

    #[inline]
    pub fn has_local_name(&self, index: u32) -> bool {
        unsafe { BinaryenFunctionHasLocalName(self.as_ptr(), index) }
    }

    // TODO: BinaryenFunctionGetLocalName, BinaryenFunctionSetLocalName

    // pub fn body(&self) -> ??

    pub fn optimize(&self, module: &Module) {
        unsafe {
            BinaryenFunctionOptimize(self.as_ptr(), module.as_ptr())
        }
    }

    // TODO: BinaryenFunctionRunPasses
}