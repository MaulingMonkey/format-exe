macro_rules! from_memory_struct {
    (
        $(
            $( #[ $($struct_meta:meta),+ $(,)? ] )*
            $struct_vis:vis struct $struct_ident:ident {
                $(
                    $( #[ $($field_meta:meta),+ $(,)? ] )*
                    $field_vis:vis $field_ident:ident : $field_ty:ty
                ),* $(,)?
            }
        )*
    ) => {
        $(
            $( #[ $($struct_meta),+ ] )*
            $struct_vis struct $struct_ident {
                $(
                    $( #[ $($field_meta),+ ] )*
                    $field_vis $field_ident : $field_ty,
                )*
            }

            impl $struct_ident {
                #[allow(dead_code)] fn impls() { // gross hack to allow multiple "Raw" structs without giving them all individual idents
                    use bytemuck::*;
                    use ::std::result::Result;

                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default)]
                    #[derive(Pod, Zeroable)]
                    pub struct Raw {
                        $( pub(crate) $field_ident : < $field_ty as $crate::FromMemory >::Raw ),*
                    }

                    impl $crate::FromMemory for $struct_ident {
                        type Raw    = Raw;
                        type Error  = std::io::Error;
                        fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> {
                            Ok(Self {
                                $(
                                    $field_ident : <$field_ty as $crate::FromMemory>::from_raw(raw.$field_ident)?
                                ),*
                            })
                        }
                    }
                }
            }
        )*
    };
}

macro_rules! from_memory_flags {
    (
        $(
            $( #[ $($struct_meta:meta),+ $(,)? ] )*
            $struct_vis:vis struct $struct_ident:ident : $struct_inner:ty {
                $(
                    $( #[ $field_meta_ident:ident $($field_meta_args:tt)* ] )*
                    const $field_ident:ident = $field_expr:expr;
                )*
            }
        )*
    ) => {
        $(
            bitflags::bitflags! {
                $( #[ $($struct_meta),+ ] )*
                $struct_vis struct $struct_ident : $struct_inner {
                    $(
                        $( #[ $field_meta_ident $($field_meta_args)* ] )*
                        const $field_ident = $field_expr ;
                    )*
                }
            }

            impl ::std::default::Default for $struct_ident {
                fn default() -> Self { Self::empty() }
            }

            impl $crate::FromMemory for $struct_ident {
                type Raw    = <$struct_inner as $crate::FromMemory>::Raw;
                type Error  = <$struct_inner as $crate::FromMemory>::Error;
                fn from_raw(raw: Self::Raw) -> Result<Self, Self::Error> {
                    Ok(Self::from_bits_truncate(<$struct_inner as $crate::FromMemory>::from_raw(raw)?))
                }
            }
        )*
    };
}
