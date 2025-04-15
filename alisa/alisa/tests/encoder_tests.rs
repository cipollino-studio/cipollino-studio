//! Tests for the binary file format encoder

mod encoder;

fn encode<F: FnOnce(&mut alisa::Encoder<Vec<u8>>) -> std::io::Result<()>>(f: F) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut encoder = alisa::Encoder::new(&mut bytes);
    f(&mut encoder).expect("encoding failed");
    bytes
}
