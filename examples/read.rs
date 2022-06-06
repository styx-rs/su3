use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("Missing path parameter");
    let raw_su3 = fs::read(path).expect("Failed to read file");
    let (_, parsed_su3) = su3::deserialise(&raw_su3).expect("Failed to parse SU3 file");

    println!("{parsed_su3:#?}");
}
