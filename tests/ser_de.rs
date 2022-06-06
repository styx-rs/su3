use su3::{ContentType, FileType, Su3};

/// Parse meeh I2P seeds file included from the Java I2P router tests
#[test]
fn meeh_i2pseeds() {
    let raw_su3 = include_bytes!("../assets/meeh_i2pseeds.su3");
    let (rest_bytes, parsed_su3) =
        Su3::deserialise(raw_su3).expect("Failed to parse I2Pseeds SU3 file");

    assert!(
        rest_bytes.is_empty(),
        "Bytes remaining: {}",
        rest_bytes.len()
    );

    assert_eq!(parsed_su3.content_type, ContentType::ReseedData);
    assert_eq!(parsed_su3.file_type, FileType::Zip);
    assert_eq!(parsed_su3.signer_id(), Ok("meeh@mail.i2p"));

    let mut reserialised = vec![0; raw_su3.len()];
    parsed_su3.serialise()((&mut *reserialised).into())
        .expect("Failed to serialise I2Pseed SU3 file");

    assert_eq!(reserialised, raw_su3);
}
