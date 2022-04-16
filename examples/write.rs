use deku::DekuUpdate;
use su3::{deku::DekuContainerWrite, Su3};

fn main() {
    // Some raw SU3 file. Doesn't matter
    let version = [0; 16];
    let mut su3 = Su3 {
        raw_version: &version,
        ..Su3::default()
    };
    su3.update().expect("Failed to update SU3 file");

    let raw_su3 = su3.to_bytes().expect("Failed to serialise SU3 file");
    println!("{raw_su3:#?}");
}
