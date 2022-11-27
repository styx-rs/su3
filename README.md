# su3

En-/decoder for the SU3 file format used by I2P for sending reseed information, updates and more.  
Deserialisation is built on top of [nom](https://docs.rs/nom) and serialisation is build on top of [cookie-factory](https://docs.rs/cookie-factory)

[Format specification](https://geti2p.net/spec/updates#su3-file-specification)

## Features

- `compression`: Helper function to decompress the content when appropriate
- `std` (enabled by default): Enabled the `std` feature on `cookie-factory` and `nom`. Uses the actual `std::io::Write` trait instead of `cookie-factory`'s polyfill

## License

This crate is licensed under the [MIT license](https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as MIT, without any additional terms or conditions.
