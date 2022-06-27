#![feature(once_cell)]
#![feature(backtrace)]
#![feature(macro_metavar_expr)]

extern crate core;

use std::fmt;
use thiserror::Error;

mod r#type;
mod module;
mod features;
mod literal;
mod external;
mod expression;
mod error;
mod tag;
mod global;
mod function;
mod export;
mod op;
pub mod settings;

use binaryen_sys as sys;
pub use r#type::*;
pub use module::*;
pub use features::*;
pub use literal::*;
pub use external::*;
pub use expression::*;
pub use error::*;
pub use tag::*;
pub use global::*;
pub use function::*;
pub use export::*;
pub use op::*;

mod generated {
    #![allow(unused)]
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
