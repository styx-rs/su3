#![allow(clippy::cast_possible_truncation)]

use crate::{Su3, MAGIC_BYTES};
use cookie_factory::{
    bytes::{be_u16, be_u64, be_u8},
    combinator::slice,
    lib::std::io::Write,
    sequence::tuple,
    GenError, SerializeFn,
};

pub fn serialise<'a, W>(su3: &'a Su3<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write + 'a,
{
    |ctx| {
        if su3.raw_version.len() < 16 {
            return Err(GenError::CustomError(1));
        }

        tuple((
            slice(MAGIC_BYTES),
            be_u8(0),
            be_u8(0),
            be_u16(su3.signature_type as u16),
            be_u16(su3.signature_type.length()),
            be_u8(0),
            be_u8(su3.raw_version.len() as u8),
            be_u8(0),
            be_u8(su3.raw_signer_id.len() as u8),
            be_u64(su3.raw_content.len() as u64),
            be_u8(0),
            be_u8(su3.file_type as u8),
            be_u8(0),
            be_u8(su3.content_type as u8),
            slice(&[0; 12]),
            slice(su3.raw_version),
            slice(su3.raw_signer_id),
            slice(su3.raw_content),
            slice(su3.raw_signature),
        ))(ctx)
    }
}
