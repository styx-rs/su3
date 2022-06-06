use su3::Su3;

fn main() {
    // Some raw SU3 file. Doesn't matter
    let version = [0; 16];
    let su3 = Su3 {
        raw_version: &version,
        ..Su3::default()
    };

    let mut buffer = [0_u8; 100];
    su3.serialise()((&mut buffer[..]).into()).expect("Failed to serialise SU3 file");
    println!("{buffer:#?}");
}
