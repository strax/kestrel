use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use binaryen_sys::*;

macro_rules! write_constant {
    ($file:expr, const $ident:ident : $type:ident = $func:ident()) => {{
        unsafe {
            let value = $func();
            writeln!($file, "pub const {}: {} = {};", stringify!($ident), stringify!($type), value).unwrap();
        }
    }}
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut file = File::create(out_dir.join("generated.rs")).unwrap();

    writeln!(file, "use binaryen_sys::*;").unwrap();

    write_constant!(file, const TYPE_NONE: BinaryenType = BinaryenTypeNone());
    write_constant!(file, const TYPE_INT32: BinaryenType = BinaryenTypeInt32());
    write_constant!(file, const TYPE_INT64: BinaryenType = BinaryenTypeInt64());
    write_constant!(file, const TYPE_FLOAT32: BinaryenType = BinaryenTypeFloat32());
    write_constant!(file, const TYPE_FLOAT64: BinaryenType = BinaryenTypeFloat64());
    write_constant!(file, const TYPE_VEC128: BinaryenType = BinaryenTypeVec128());
    write_constant!(file, const TYPE_FUNCREF: BinaryenType = BinaryenTypeFuncref());
    write_constant!(file, const TYPE_EXTERNREF: BinaryenType = BinaryenTypeExternref());
    write_constant!(file, const TYPE_ANYREF: BinaryenType = BinaryenTypeAnyref());
    write_constant!(file, const TYPE_EQREF: BinaryenType = BinaryenTypeEqref());
    write_constant!(file, const TYPE_I31REF: BinaryenType = BinaryenTypeI31ref());
    write_constant!(file, const TYPE_DATAREF: BinaryenType = BinaryenTypeDataref());
    write_constant!(file, const TYPE_UNREACHABLE: BinaryenType = BinaryenTypeUnreachable());

    write_constant!(file, const FEATURE_MVP: BinaryenFeatures = BinaryenFeatureMVP());
    write_constant!(file, const FEATURE_ATOMICS: BinaryenFeatures = BinaryenFeatureAtomics());
    write_constant!(file, const FEATURE_BULK_MEMORY: BinaryenFeatures = BinaryenFeatureBulkMemory());
    write_constant!(file, const FEATURE_MUTABLE_GLOBALS: BinaryenFeatures = BinaryenFeatureMutableGlobals());
    write_constant!(file, const FEATURE_NONTRAPPING_FP_TO_INT: BinaryenFeatures = BinaryenFeatureNontrappingFPToInt());
    write_constant!(file, const FEATURE_SIGN_EXT: BinaryenFeatures = BinaryenFeatureSignExt());
    write_constant!(file, const FEATURE_SIMD128: BinaryenFeatures = BinaryenFeatureSIMD128());
    write_constant!(file, const FEATURE_EXCEPTION_HANDLING: BinaryenFeatures = BinaryenFeatureExceptionHandling());
    write_constant!(file, const FEATURE_TAIL_CALL: BinaryenFeatures = BinaryenFeatureTailCall());
    write_constant!(file, const FEATURE_REFERENCE_TYPES: BinaryenFeatures = BinaryenFeatureReferenceTypes());
    write_constant!(file, const FEATURE_MULTIVALUE: BinaryenFeatures = BinaryenFeatureMultivalue());
    write_constant!(file, const FEATURE_GC: BinaryenFeatures = BinaryenFeatureGC());
    write_constant!(file, const FEATURE_MEMORY64: BinaryenFeatures = BinaryenFeatureMemory64());
    write_constant!(file, const FEATURE_TYPED_FUNCTION_REFERENCES: BinaryenFeatures = BinaryenFeatureTypedFunctionReferences());
    write_constant!(file, const FEATURE_RELAXED_SIMD: BinaryenFeatures = BinaryenFeatureRelaxedSIMD());
    write_constant!(file, const FEATURE_EXTENDED_CONST: BinaryenFeatures = BinaryenFeatureExtendedConst());
    write_constant!(file, const FEATURE_ALL: BinaryenFeatures = BinaryenFeatureAll());

    write_constant!(file, const EXTERNAL_FUNCTION: BinaryenExternalKind = BinaryenExternalFunction());
    write_constant!(file, const EXTERNAL_TABLE: BinaryenExternalKind = BinaryenExternalTable());
    write_constant!(file, const EXTERNAL_MEMORY: BinaryenExternalKind = BinaryenExternalMemory());
    write_constant!(file, const EXTERNAL_GLOBAL: BinaryenExternalKind = BinaryenExternalGlobal());
    write_constant!(file, const EXTERNAL_TAG: BinaryenExternalKind = BinaryenExternalTag());
}