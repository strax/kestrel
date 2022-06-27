use std::cell::UnsafeCell;
use crate::sys::*;
use crate::{Expression, Type};
use std::ffi::CStr;

pub struct GlobalDescriptor<'ctx, 'name> {
    pub name: &'name CStr,
    pub type_: Type,
    pub mutable: bool,
    pub initializer: &'ctx dyn Expression
}

#[repr(transparent)]
pub struct Global(UnsafeCell<BinaryenGlobal>);

impl Global {
    #[inline]
    const fn as_ptr(&self) -> *mut BinaryenGlobal {
        self.0.get()
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(BinaryenGlobalGetName(self.as_ptr())) }
    }

    #[inline]
    pub fn is_mutable(&self) -> bool {
        unsafe { BinaryenGlobalIsMutable(self.as_ptr()) }
    }

    #[inline]
    pub fn type_(&self) -> Type {
        unsafe { Type::from_raw(BinaryenGlobalGetType(self.as_ptr())) }
    }

    // pub fn initializer(&self) -> ??
}