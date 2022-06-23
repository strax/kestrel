use crate::sys::*;
use paste::paste;

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

#[doc(hidden)]
pub trait Setting: Sealed {
    type Value;

    fn get() -> Self::Value;
    fn set(value: Self::Value);
}

macro_rules! setting {
    ($name:ident, $value:ty) => {
        macro_rules! getter {
            ($$name:ident) => {
                paste! {
                    [<BinaryenGet $$name>]
                }
            };
        }

        macro_rules! setter {
            ($$name:ident) => {
                paste! {
                    [<BinaryenSet $$name>]
                }
            };
        }

        setting!($name, $value, getter!($name), setter!($name));
    };
    ($name:ident, $value:ty, $getter:expr, $setter:expr) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $name;
        impl Sealed for $name {}
        impl Setting for $name {
            type Value = $value;

            #[inline]
            fn get() -> Self::Value {
                unsafe {
                    let getter = $getter;
                    getter()
                }
            }

            #[inline]
            fn set(value: Self::Value) {
                unsafe {
                    let setter = $setter;
                    setter(value);
                }
            }
        }
    };
}

setting!(FastMath, bool);
setting!(AllowInliningFunctionsWithLoops, bool);
setting!(LowMemoryUnused, bool);
setting!(ZeroFilledMemory, bool);
setting!(FlexibleInlineMaxSize, u32);
setting!(AlwaysInlineMaxSize, u32);
setting!(OneCallerInlineMaxSize, u32);
setting!(OptimizeLevel, i32);
setting!(ShrinkLevel, i32);
setting!(ColorsEnabled, bool, BinaryenAreColorsEnabled, BinaryenSetColorsEnabled);

#[inline(always)]
pub fn enable<T: Setting<Value = bool>>(setting: T) {
    set(setting, true);
}

#[inline(always)]
pub fn disable<T: Setting<Value = bool>>(setting: T) {
    set(setting, false);
}

#[inline(always)]
pub fn is_enabled<T: Setting<Value = bool>>(setting: T) -> bool {
    get(setting)
}

#[inline(always)]
pub fn get<T: Setting>(_: T) -> T::Value {
    T::get()
}

#[inline(always)]
pub fn set<T: Setting>(_: T, value: T::Value) {
    T::set(value)
}