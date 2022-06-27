use std::cell::UnsafeCell;
use std::ffi::CStr;
use std::ptr;
use crate::sys::*;
use crate::Type;

pub struct TagDescriptor<'name> {
    pub name: &'name CStr,
    pub params: Type,
    pub results: Type
}

#[repr(transparent)]
pub struct Tag(UnsafeCell<BinaryenTag>);

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}
impl Eq for Tag {}