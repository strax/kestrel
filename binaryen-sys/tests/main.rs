use binaryen_sys::*;

#[test]
fn test_linkage() {
    unsafe {
        BinaryenSetColorsEnabled(true);
        assert_eq!(BinaryenAreColorsEnabled(), true);
        BinaryenSetColorsEnabled(false);
        assert_eq!(BinaryenAreColorsEnabled(), false);
    }
}