/// Implement `TryFrom` some integer type for an enum
///
/// Usage:
///
/// ```ignore
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
            type Error = ::nom::Err<::nom::error::Error<&'static [u8]>>;

            fn try_from(num: $num_type) -> Result<Self, Self::Error> {
                use ::nom::{Err, error::{Error, ErrorKind}};

                match num {
                    $(
                        $value => Ok(Self::$variant),
                    )*
                    _ => Err(Err::Failure(Error::new(&[], ErrorKind::Digit)))
                }
            }
        }
    }
}
