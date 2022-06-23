use std::ffi::CString;
use std::ptr::{NonNull, null_mut};
use crate::{Module, Type};
use crate::sys::*;

pub trait Expression {
    #[doc(hidden)]
    fn as_raw_mut(&mut self) -> &mut BinaryenExpression;
}

macro_rules! expression {
    ($name:ident) => {
        #[repr(transparent)]
        pub struct $name(BinaryenExpression);

        impl Expression for $name {
            #[inline]
            fn as_raw_mut(&mut self) -> &mut BinaryenExpression {
                &mut self.0
            }
        }
    }
}

expression!(Block);

impl Block {
    pub fn new<'a>(module: &'a Module, name: &str, type_: Type) -> &'a mut Block {
        let name = CString::new(name).unwrap();
        unsafe {
            // SAFETY: Block is repr(transparent) so *mut BinaryenExpression can be cast to *mut Block.
            // The BinaryenExpression is arena allocated and lives as long as the Module
            NonNull::new(BinaryenBlock(module.as_ptr(), name.as_ptr(), null_mut(), 0, type_.to_raw()))
                .unwrap()
                .cast()
                .as_mut()
        }
    }
}
