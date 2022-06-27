use std::ffi::{CStr, CString};
use std::ptr;
use crate::sys::*;
use std::ptr::NonNull;
use crate::{Expression, Features, Function, FunctionDescriptor, Tag, TagDescriptor, Type};
use crate::global::{Global, GlobalDescriptor};
use tinyvec::ArrayVec;

#[repr(transparent)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Module(NonNull<BinaryenModule>);

impl Module {
    pub fn parse(text: &CStr) -> Module {
        NonNull::new(unsafe { BinaryenModuleParse(text.as_ptr()) })
            .map(Module)
            .unwrap()
    }
}

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

    #[inline]
    pub fn features(&self) -> Features {
        unsafe {
            Features::from_bits_unchecked(BinaryenModuleGetFeatures(self.as_ptr()))
        }
    }

    #[inline]
    pub fn set_features(&self, features: Features) {
        unsafe {
            BinaryenModuleSetFeatures(self.as_ptr(), features.bits());
        }
    }

    #[inline]
    pub fn print(&self) {
        unsafe { BinaryenModulePrint(self.as_ptr()) }
    }

    #[inline]
    pub fn validate(&self) -> bool {
        unsafe { BinaryenModuleValidate(self.as_ptr()) }
    }

    #[inline]
    pub fn optimize(&self) {
        unsafe { BinaryenModuleOptimize(self.as_ptr()) }
    }

    #[inline]
    pub fn update_maps(&self) {
        unsafe { BinaryenModuleUpdateMaps(self.as_ptr()) }
    }

    #[inline]
    pub fn auto_drop(&self) {
        unsafe { BinaryenModuleAutoDrop(self.as_ptr()) }
    }

    #[inline]
    pub fn interpret(&self) {
        unsafe { BinaryenModuleInterpret(self.as_ptr()) }
    }

    pub fn add_custom_section(&self, name: &CStr, contents: &[u8]) {
        assert!(contents.len() <= (u32::MAX as usize));
        unsafe {
            BinaryenAddCustomSection(self.as_ptr(), name.as_ptr(), contents.as_ptr().cast(), contents.len() as _)
        }
    }

    #[inline]
    pub fn num_memory_segments(&self) -> u32 {
        unsafe { BinaryenGetNumMemorySegments(self.as_ptr()) }
    }

    #[inline]
    pub fn has_memory(&self) -> bool {
        unsafe { BinaryenHasMemory(self.as_ptr()) }
    }

    #[inline]
    pub fn memory_get_initial(&self) -> u32 {
        unsafe { BinaryenMemoryGetInitial(self.as_ptr()) }
    }

    #[inline]
    pub fn memory_has_max(&self) -> bool {
        unsafe { BinaryenMemoryHasMax(self.as_ptr()) }
    }

    #[inline]
    pub fn memory_get_max(&self) -> u32 {
        unsafe { BinaryenMemoryGetMax(self.as_ptr()) }
    }

    #[inline]
    pub fn memory_is_shared(&self) -> bool {
        unsafe { BinaryenMemoryIsShared(self.as_ptr()) }
    }

    pub fn add_tag(&self, desc: &TagDescriptor) -> &Tag {
        unsafe {
            let ptr = BinaryenAddTag(
                self.as_ptr(),
                desc.name.as_ptr(),
                desc.params.to_raw(),
                desc.results.to_raw()
            );
            NonNull::new(ptr).expect("BinaryenAddTag returned a NULL pointer").cast().as_ref()
        }
    }

    pub fn get_tag(&self, name: &CStr) -> Option<&Tag> {
        unsafe {
            let ptr = BinaryenGetTag(self.as_ptr(), name.as_ptr());
            NonNull::new(ptr).map(|ptr| ptr.cast().as_ref())
        }
    }

    pub fn remove_tag(&self, name: &CStr) {
        unsafe {
            BinaryenRemoveTag(self.as_ptr(), name.as_ptr());
        }
    }

    pub fn add_global<'ctx>(&'ctx self, desc: &GlobalDescriptor<'ctx, '_>) -> &'ctx Global {
        unsafe {
            let ptr = BinaryenAddGlobal(
                self.as_ptr(),
                desc.name.as_ptr(),
                desc.type_.to_raw(),
                desc.mutable,
                desc.initializer.as_raw().as_ptr()
            );
            NonNull::new(ptr).expect("BinaryenAddGlobal returned a null pointer").cast().as_ref()
        }
    }

    // TODO
    // pub fn set_memory(&mut self, initial: u32, maximum: u32, export_name: Option<&CStr>, segments: &[MemorySegment])

    //#region Functions
    pub fn add_function<'a>(&self, desc: &FunctionDescriptor<'a, '_>) -> &'a mut Function {
        unsafe {
            let ptr = BinaryenAddFunction(
                self.as_ptr(),
                desc.name.as_ptr(),
                desc.params.to_raw(),
                desc.results.to_raw(),
                // SAFETY: The data behind the pointer is not mutated by `BinaryenAddFunction` but it is not marked const
                desc.vars.as_ptr() as *const BinaryenType as *mut BinaryenType,
                desc.vars.len() as u32,
                desc.body.as_raw().as_ptr()
            );
            assert!(!ptr.is_null());
            &mut *ptr.cast()
        }
    }

    #[inline]
    pub fn get_function(&self, name: &CStr) -> Option<&Function> {
        unsafe {
            NonNull::new(BinaryenGetFunction(self.as_ptr(), name.as_ptr()))
                .map(|ptr| ptr.cast().as_ref())
        }
    }

    #[inline]
    pub fn get_function_by_index(&self, index: u32) -> &Function {
        unsafe {
            &*BinaryenGetFunctionByIndex(self.as_ptr(), index).cast()
        }
    }

    #[inline]
    pub fn remove_function(&self, name: &CStr) {
        unsafe { BinaryenRemoveFunction(self.as_ptr(), name.as_ptr()) }
    }

    #[inline]
    pub fn num_functions(&self) -> u32 {
        unsafe { BinaryenGetNumFunctions(self.as_ptr()) }
    }

    pub fn add_function_export(&self, internal_name: &CStr, external_name: &CStr) {
        unsafe { BinaryenAddFunctionExport(self.as_ptr(), internal_name.as_ptr(), external_name.as_ptr()); }
    }
    //#endregion
}

impl Drop for Module {
    #[inline]
    fn drop(&mut self) {
        unsafe { BinaryenModuleDispose(self.0.as_ptr()) }
    }
}
