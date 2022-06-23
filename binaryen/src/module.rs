use crate::sys::*;
use std::ptr::NonNull;

#[repr(transparent)]
pub struct Module(NonNull<BinaryenModule>);

impl Module {
    #[inline]
    pub fn new() -> Module {
        let ptr = unsafe { BinaryenModuleCreate() };
        NonNull::new(ptr).map(Module).expect("BinaryenModuleCreate returned a null pointer")
    }

    #[inline]
    pub(crate) const fn as_ptr(&self) -> *mut BinaryenModule {
        self.0.as_ptr()
    }
}

impl Drop for Module {
    #[inline]
    fn drop(&mut self) {
        unsafe { BinaryenModuleDispose(self.0.as_ptr()) }
    }
}