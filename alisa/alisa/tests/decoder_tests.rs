
mod decoder;

fn value_skipped(bytes: &[u8]) -> bool {
    // We might encounter a bug where `skip_value` consumes too many bytes.
    // To test for this, we add some magic bytes to the end of the buffer and make sure they aren't consumed
    let test_header = b"ENDCAP";
    let mut test_bytes = Vec::new();
    test_bytes.extend_from_slice(bytes);
    test_bytes.extend_from_slice(test_header);

    let mut decoder = alisa::Decoder::new(&test_bytes);
    decoder.skip_value();

    if decoder.remaining_bytes() != test_header {
        eprintln!("REMAINING BYTES: {:?}", decoder.remaining_bytes());
    }

    decoder.remaining_bytes() == test_header
}
