#[macro_use]
build_hash! { BLAKE3 (reader) {
        let digestbytes = 32;
        let buf_size = 32768;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_size);
        let mut hash_state = blake3::Hasher::new();
        let mut limited_reader = reader.take(buf_size as u64);
        loop {
            match limited_reader.read_to_end(&mut buf) {
                Ok(0) => break,
                Ok(_) => {
                    hash_state.update(&buf[..]);
                    buf.clear();
                    limited_reader = limited_reader.into_inner().take(buf_size as u64);
                }
                Err(_err) => return "00".repeat(digestbytes),
            }
        }
        hash_state.finalize().to_hex().to_string()
}}
