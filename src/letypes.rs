use bytemuck::*;

use std::fmt::{self, *};



macro_rules! letypes {
    ($(pub struct $le:ident($inner:ident);)*) => {
        $(
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            #[derive(Clone, Copy, Default, PartialEq, Eq)]
            #[derive(Pod, Zeroable)]
            pub struct $le($inner);

            impl $le {
                pub fn to_le(self) -> $inner {
                    self.0.to_le()
                }

                pub fn into(self) -> $inner {
                    self.to_le()
                }
            }

            impl From<$inner> for $le {
                fn from(value: $inner) -> $le {
                    $le($inner::from_le(value))
                }
            }

            impl From<$le> for $inner {
                fn from(value: $le) -> $inner {
                    value.to_le()
                }
            }

            // TODO: PartialOrd/Ord/Hash ?

            impl Display for $le {
                fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
                    Display::fmt(&self.to_le(), fmt)
                }
            }

            impl Debug for $le {
                fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
                    Debug::fmt(&self.to_le(), fmt)
                }
            }
        )*
    };
}

letypes! {
    pub struct u8le(u8);
    pub struct u16le(u16);
    pub struct u32le(u32);
    pub struct u64le(u64);
    pub struct u128le(u128);

    pub struct i8le(i8);
    pub struct i16le(i16);
    pub struct i32le(i32);
    pub struct i64le(i64);
    pub struct i128le(i128);
}
