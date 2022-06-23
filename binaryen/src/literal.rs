use crate::sys::*;

#[derive(Copy, Clone)]
pub enum Literal {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    Vec128([u8; 16])
    // TODO: func
}

#[doc(hidden)]
impl Into<BinaryenLiteral> for Literal {
    fn into(self) -> BinaryenLiteral {
        unsafe {
            match self {
                Literal::Int32(x) => BinaryenLiteralInt32(x),
                Literal::Int64(x) => BinaryenLiteralInt64(x),
                Literal::Float32(x) => BinaryenLiteralFloat32(x),
                Literal::Float64(x) => BinaryenLiteralFloat64(x),
                Literal::Vec128(x) => BinaryenLiteralVec128(x.as_ptr()),
            }
        }
    }
}