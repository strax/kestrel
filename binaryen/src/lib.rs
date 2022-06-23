#![feature(once_cell)]
mod r#type;
mod module;
mod features;
mod literal;
mod external;
mod expression;
pub mod settings;

use binaryen_sys as sys;
pub use r#type::*;
pub use module::*;
pub use features::*;
pub use literal::*;
pub use external::*;
pub use expression::*;

mod generated {
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

