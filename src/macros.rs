/// Implement `TryFrom` some integer type for an enum
///
/// Usage:
///
/// ```
/// try_from_number! {
///     pub enum Test: u8 {
///         A = 0x01,
///         B = 0x02,
///     }
/// }
/// ```
macro_rules! try_from_number {
    (
        $(
            #[$($top_attribute:tt)*]
        )*
        pub enum $enum_name:ident : $num_type:ty {
            $(
                $(
                    #[$($field_attribute:tt)*]
                )*
                $variant:ident = $value:literal,
            )*
        }
    ) => {
        $(
            #[$($top_attribute)*]
        )*
        pub enum $enum_name {
            $(
                $(
                    #[$($field_attribute)*]
                )*
                $variant = $value,
            )*
        }

        impl ::core::convert::TryFrom<$num_type> for $enum_name {
            type Error = ();

            fn try_from(num: $num_type) -> Result<Self, Self::Error> {
                match num {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(())
                }
            }
        }
    }
}
