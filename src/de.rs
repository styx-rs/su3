use crate::{Su3, MAGIC_BYTES};
use nom::{
    bytes::complete::{tag, take},
    combinator::value,
    error::Error,
    number::complete::{be_u16, be_u64, be_u8},
    sequence::tuple,
    IResult, Parser,
};

/// Read some data via a parser and discard its output
fn skip<P, I, O>(parser: P) -> impl Parser<I, (), Error<I>>
where
    P: Parser<I, O, Error<I>>,
{
    value((), parser)
}

pub fn deserialise(data: &[u8]) -> IResult<&[u8], Su3<'_>> {
    let (
        rest,
        (
            (),
            (),
            (),
            signature_type,
            signature_length,
            (),
            version_length,
            (),
            signer_id_length,
            content_length,
            (),
            file_type,
            (),
            content_type,
            (),
        ),
    ) = tuple((
        skip(tag(MAGIC_BYTES)),
        skip(be_u8),
        skip(be_u8),
        be_u16,
        be_u16,
        skip(be_u8),
        be_u8,
        skip(be_u8),
        be_u8,
        be_u64,
        skip(be_u8),
        be_u8,
        skip(be_u8),
        be_u8,
        skip(take(12_usize)),
    ))(data)?;

    let (rest, (raw_version, raw_signer_id, raw_content, raw_signature)) = tuple((
        take(version_length),
        take(signer_id_length),
        take(content_length),
        take(signature_length),
    ))(rest)?;

    let su3 = Su3 {
        signature_type: signature_type.try_into()?,
        file_type: file_type.try_into()?,
        content_type: content_type.try_into()?,
        raw_version,
        raw_signer_id,
        raw_content,
        raw_signature,
    };

    Ok((rest, su3))
}
