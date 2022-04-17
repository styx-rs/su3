use crate::{ContentType, FileType, Su3};
use deku::DekuContainerRead;

/// Parse meeh I2P seeds file included from the Java I2P router tests
#[test]
fn meeh_i2pseeds() {
    let raw_su3 = include_bytes!("../assets/meeh_i2pseeds.su3");
    let ((rest_bytes, _), parsed_su3) =
        Su3::from_bytes((raw_su3, 0)).expect("Failed to parse I2Pseeds SU3 file");

    assert!(rest_bytes.is_empty());

    assert_eq!(parsed_su3.content_type, ContentType::ReseedData);
    assert_eq!(parsed_su3.file_type, FileType::Zip);
    assert_eq!(parsed_su3.signer_id(), Ok("meeh@mail.i2p"));
}
