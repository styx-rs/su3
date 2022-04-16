//!
//! En-/decoder for the SU3 file format used by I2P for sending reseed information, updates and more
//!
//! [Format specification](https://geti2p.net/spec/updates#su3-file-specification)
//!

#![forbid(missing_docs, unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

pub extern crate deku;

use deku::{
    DekuContainerRead, DekuContainerWrite, DekuEnumExt, DekuError, DekuRead, DekuUpdate, DekuWrite,
};
use flate2::read::GzDecoder;
use std::{
    borrow::Cow,
    io::{self, Read},
    str::{self, Utf8Error},
};

/// Minimum length of the version field
const MIN_VERSION_LENGTH: u8 = 16;

/// Content type
#[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq, Eq, PartialOrd, Ord)]
#[deku(ctx = "endian: deku::ctx::Endian", endian = "endian", type = "u8")]
pub enum ContentType {
    /// Unknown content type
    Unknown = 0x00,

    /// Router update
    RouterUpdate,

    /// Plugin (update)
    Plugin,

    /// Reseed data
    ReseedData,

    /// News feed
    NewsFeed,

    /// Blocklist feed
    BlocklistFeed,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}

/// File type
#[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq, Eq, PartialOrd, Ord)]
#[deku(ctx = "endian: deku::ctx::Endian", endian = "endian", type = "u8")]
pub enum FileType {
    /// ZIP file
    Zip = 0x00,

    /// XML file
    Xml,

    /// HTML file
    Html,

    /// GZ compressed XML file
    XmlGz,

    /// GZ compressed TXT file
    TxtGz,

    /// DMG file
    Dmg,

    /// EXE file
    Exe,
}

impl Default for FileType {
    fn default() -> Self {
        Self::Zip
    }
}

/// Signature type
#[derive(Clone, Debug, DekuRead, DekuWrite, PartialEq, Eq, PartialOrd, Ord)]
#[deku(ctx = "endian: deku::ctx::Endian", endian = "endian", type = "u16")]
pub enum SignatureType {
    /// DSA-SHA1
    DsaSha1 = 0x0000,

    /// ECDSA-SHA256-P256
    EcdsaSha256P256 = 0x0001,

    /// ECDSA-SHA384-P384
    EcdsaSha384P384 = 0x0002,

    /// ECDSA-SHA512-P521
    EcdsaSha512P521 = 0x0003,

    /// RSA-SHA256-2048
    RsaSha2562048 = 0x0004,

    /// RSA-SHA384-3072
    RsaSha3843072 = 0x0005,

    /// RSA-SHA512-4096
    RsaSha5124096 = 0x0006,

    /// EdDSA-SHA512-Ed25519ph
    EddsaSha512Ed25519ph = 0x0008,
}

impl SignatureType {
    /// Get the signature length in bytes
    ///
    /// Source: <https://geti2p.net/spec/common-structures#type-signature>
    #[must_use]
    pub fn length(&self) -> u16 {
        match self {
            Self::DsaSha1 => 40,
            Self::EcdsaSha256P256 | Self::EddsaSha512Ed25519ph => 64,
            Self::EcdsaSha384P384 => 96,
            Self::EcdsaSha512P521 => 132,
            Self::RsaSha2562048 => 256,
            Self::RsaSha3843072 => 384,
            Self::RsaSha5124096 => 512,
        }
    }
}

impl Default for SignatureType {
    fn default() -> Self {
        Self::DsaSha1
    }
}

/// Typed representation of an SU3 file
#[derive(Clone, Debug, Default, DekuRead, DekuWrite, PartialEq, Eq, PartialOrd, Ord)]
#[deku(endian = "big", magic = b"I2Psu3")]
pub struct Su3<'a> {
    /// Unused field
    pub unused_0: u8,

    /// SU3 file format version
    pub format_version: u8,

    /// Signature type
    pub signature_type: SignatureType,

    /// Signature length
    #[deku(update = "self.signature_type.length()")]
    pub signature_length: u16,

    /// Unused field
    pub unused_1: u8,

    /// Version length (in bytes; includes padding)
    ///
    /// Has to be at least 16
    #[deku(
        assert = "*version_length >= MIN_VERSION_LENGTH",
        update = "self.raw_version.len()"
    )]
    pub version_length: u8,

    /// Unused field
    pub unused_2: u8,

    /// Signer ID length (in bytes)
    pub signer_id_length: u8,

    /// Content length (not including header or signature)
    #[deku(update = "self.raw_content.len()")]
    pub content_length: u64,

    /// Unused field
    pub unused_3: u8,

    /// File type
    pub file_type: FileType,

    /// Unused field
    pub unused_4: u8,

    /// Content type
    pub content_type: ContentType,

    /// Unused field
    pub unused_5: [u8; 12],

    /// Version (UTF-8 padded with null bytes)
    ///
    /// At least 16 bytes in length (length specified by field `version_length`)
    #[deku(count = "version_length")]
    pub raw_version: &'a [u8],

    /// Signer ID (eg. "zzz@mail.i2p"; UTF-8 encoded; no padding, length specified by field `signer_id_length`)
    #[deku(count = "signer_id_length")]
    pub raw_signer_id: &'a [u8],

    /// Raw content
    #[deku(count = "content_length")]
    pub raw_content: &'a [u8],

    /// Signature (length specified by field `signature_length`)
    ///
    /// The signature covers the everything preceding this field
    #[deku(count = "signature_length")]
    pub raw_signature: &'a [u8],
}

impl<'a> Su3<'a> {
    /// Return the decompressed representation of the content
    ///
    /// Note: This will only decompress the `TxtGz` and `XmlGz` types. ZIP files are not handled
    ///
    /// # Errors
    ///
    /// Returns an IO error in case the decompression of the GZ compressed content fails
    pub fn decompressed_content(&self) -> io::Result<Cow<'a, [u8]>> {
        let content = match self.file_type {
            FileType::TxtGz | FileType::XmlGz => {
                let mut gz = GzDecoder::new(self.raw_content);

                let mut decompressed_content = Vec::with_capacity(self.raw_content.len());
                gz.read_to_end(&mut decompressed_content)?;

                Cow::Owned(decompressed_content)
            }
            _ => Cow::Borrowed(self.raw_content),
        };

        Ok(content)
    }

    /// Signer ID in form of a string slice
    ///
    /// # Errors
    ///
    /// An error occurs when the signer ID field of the file is not valid UTF-8
    pub fn signer_id(&self) -> Result<&'a str, Utf8Error> {
        str::from_utf8(self.raw_signer_id)
    }

    /// Version in form of a string slice
    ///
    /// # Errors
    ///
    /// An error occurs when the version field of the file is not valid UTF-8
    pub fn version(&self) -> Result<&'a str, Utf8Error> {
        str::from_utf8(self.raw_version)
    }
}
