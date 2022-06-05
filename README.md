# su3


En-/decoder for the SU3 file format used by I2P for sending reseed information, updates and more

[Format specification](https://geti2p.net/spec/updates#su3-file-specification)

Example:

```rust
let (_, parsed_su3) = Su3::from_bytes((su3_data, 0)).expect("Failed to parse SU3 file");
let content = parsed_su3.content().expect("Failed to decompress content");
```


License: MIT
