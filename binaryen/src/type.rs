use std::mem::transmute;
use crate::sys::*;
use crate::generated::*;

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Type(BinaryenType);

impl Type {
    pub const NONE: Type = Type(TYPE_NONE);
    pub const INT32: Type = Type(TYPE_INT32);
    pub const INT64: Type = Type(TYPE_INT64);
    pub const FLOAT32: Type = Type(TYPE_FLOAT32);
    pub const FLOAT64: Type = Type(TYPE_FLOAT64);
    pub const VEC128: Type = Type(TYPE_VEC128);
    pub const FUNCREF: Type = Type(TYPE_FUNCREF);
    pub const EXTERNREF: Type = Type(TYPE_EXTERNREF);
    pub const DATAREF: Type = Type(TYPE_DATAREF);
    pub const ANYREF: Type = Type(TYPE_ANYREF);
    pub const I31REF: Type = Type(TYPE_I31REF);
    pub const EQREF: Type = Type(TYPE_EQREF);
    pub const UNREACHABLE: Type = Type(TYPE_UNREACHABLE);
}

impl Type {
    #[inline]
    pub(crate) const fn from_raw(raw: BinaryenType) -> Type {
        Type(raw)
    }

    pub fn new(value_types: &[Type]) -> Type {
        // Unrealistic to hit over u32::MAX value types in a single call...
        debug_assert!(value_types.len() <= (u32::MAX as usize), "maximum amount of value types exceeded");
        unsafe {
            let ptr = value_types.as_ptr();
            let len = value_types.len();
            Type(BinaryenTypeCreate(ptr as *mut _, len as BinaryenIndex))
        }
    }

    #[inline]
    pub fn arity(self) -> u32 {
        unsafe {
            BinaryenTypeArity(self.0)
        }
    }

    #[inline]
    pub fn expand(self) -> Vec<Type> {
        let mut out = Vec::with_capacity(self.arity() as usize);
        unsafe {
            BinaryenTypeExpand(self.0, out.as_mut_ptr());
            // Since Type is repr(transparent) and Copy, a Vec of BinaryenTypes is identical to a Vec of Types.
            transmute(out)
        }
    }

    #[inline]
    pub(crate) const fn to_raw(self) -> BinaryenType {
        self.0
    }
}

impl From<(Type, Type)> for Type {
    fn from((t1, t2): (Type, Type)) -> Self {
        Type::new(&[t1, t2])
    }
}

impl From<(Type, Type, Type)> for Type {
    fn from((t1, t2, t3): (Type, Type, Type)) -> Self {
        Type::new(&[t1, t2, t3])
    }
}

impl From<(Type, Type, Type, Type)> for Type {
    fn from((t1, t2, t3, t4): (Type, Type, Type, Type)) -> Self {
        Type::new(&[t1, t2, t3, t4])
    }
}