use crate::sys::*;
use crate::generated::*;
use bitflags::bitflags;

bitflags! {
    pub struct Features : BinaryenFeatures {
        const MVP = FEATURE_MVP;
        const ATOMICS = FEATURE_ATOMICS;
        const BULK_MEMORY = FEATURE_BULK_MEMORY;
        const MUTABLE_GLOBALS = FEATURE_MUTABLE_GLOBALS;
        const NONTRAPPING_FP_TO_INT = FEATURE_NONTRAPPING_FP_TO_INT;
        const SIGN_EXT = FEATURE_SIGN_EXT;
        const SIMD128 = FEATURE_SIMD128;
        const EXCEPTION_HANDLING = FEATURE_EXCEPTION_HANDLING;
        const TAIL_CALL = FEATURE_TAIL_CALL;
        const REFERENCE_TYPES = FEATURE_REFERENCE_TYPES;
        const MULTIVALUE = FEATURE_MULTIVALUE;
        const GC = FEATURE_GC;
        const MEMORY64 = FEATURE_MEMORY64;
        const TYPED_FUNCTION_REFERENCES = FEATURE_TYPED_FUNCTION_REFERENCES;
        const RELAXED_SIMD = FEATURE_RELAXED_SIMD;
        const EXTENDED_CONST = FEATURE_EXTENDED_CONST;
        // const ALL = FEATURE_ALL;
    }
}
