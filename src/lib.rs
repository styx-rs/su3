#![doc = include_str!("../README.md")]

//!
//! # Examples
//!
//! ```
//! # use su3::Su3;
//! # let su3_data = include_bytes!("../assets/meeh_i2pseeds.su3");
//! let (_, parsed_su3) = su3::deserialise(su3_data).expect("Failed to parse SU3 file");
//! ```
//!

#![no_std]
#![forbid(missing_docs, rust_2018_idioms, unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]

use core::str::{self, Utf8Error};

mod de;
#[macro_use]
mod macros;
mod ser;

/// Magic bytes
const MAGIC_BYTES: &[u8] = b"I2Psu3";

pub use self::{de::deserialise, ser::serialise};

/// Minimum length of the version field
pub const MIN_VERSION_LENGTH: u8 = 16;

try_from_number! {
    /// Content type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ContentType: u8 {
        /// Unknown content type
        Unknown = 0x00,

        /// Router update
        RouterUpdate = 0x01,

        /// Plugin (update)
        Plugin = 0x02,

        /// Reseed data
        ReseedData = 0x03,

        /// News feed
        NewsFeed = 0x04,

        /// Blocklist feed
        BlocklistFeed = 0x05,
    }
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Unknown
    }
}

try_from_number! {
    /// File type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FileType: u8 {
        /// ZIP file
        Zip = 0x00,

        /// XML file
        Xml = 0x01,

        /// HTML file
        Html = 0x02,

        /// GZ compressed XML file
        XmlGz = 0x03,

        /// GZ compressed TXT file
        TxtGz = 0x04,

        /// DMG file
        Dmg = 0x05,

        /// EXE file
        Exe = 0x06,
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Zip
    }
}

try_from_number! {
    /// Signature type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum SignatureType: u16 {
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
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Su3<'a> {
    /// Signature type
    pub signature_type: SignatureType,

    /// File type
    pub file_type: FileType,

    /// Content type
    pub content_type: ContentType,

    /// Version (UTF-8 padded with null bytes)
    ///
    /// At least 16 bytes in length (length specified by field `version_length`)
    pub raw_version: &'a [u8],

    /// Signer ID (eg. "zzz@mail.i2p"; UTF-8 encoded; no padding, length specified by field `signer_id_length`)
    pub raw_signer_id: &'a [u8],

    /// Raw content
    pub raw_content: &'a [u8],

    /// Signature (length specified by field `signature_length`)
    ///
    /// The signature covers the everything preceding this field
    pub raw_signature: &'a [u8],
}

impl<'a> Su3<'a> {
    /// Return the possibly decompressed representation of the content
    ///
    /// Note: This will only decompress the `TxtGz` and `XmlGz` types. ZIP files are not handled
    ///
    /// # Errors
    ///
    /// Returns an IO error in case the decompression of the GZ compressed content fails
    #[cfg_attr(docsrs, doc(cfg(feature = "flate2")))]
    #[cfg(feature = "flate2")]
    pub fn content(&self) -> std::io::Result<std::borrow::Cow<'a, [u8]>> {
        use flate2::read::GzDecoder;

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

    /// Version in form of a string slice (without the null bytes)
    ///
    /// # Errors
    ///
    /// An error occurs when the version field of the file is not valid UTF-8
    pub fn version(&self) -> Result<&'a str, Utf8Error> {
        str::from_utf8(self.raw_version).map(|version| version.trim_matches(|r#char| char == '\0'))
    }
}
