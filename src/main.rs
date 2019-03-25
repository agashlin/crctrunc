//! Check if a file could be truncated with zeroes to match the given `target_checksum`.
//! Bytes are kept from the start of the file in chunks of `chunk_size`.

extern crate crc32fast;

use std::env;
use std::fs::File;
use std::io::Read;

use crc32fast::Hasher;

fn main() {
    let args: Vec<_> = env::args_os().collect();
    if args.len() != 4 {
        eprintln!(
            r"Usage: crctrunc file target_checksum chunk_size

All values are hexadecimal.
Example: crctrunc omni.ja 326fbb3c 10000"
        );
        return;
    }
    let mut in_file = File::open(&args[1]).expect("couldn't open file");
    let in_len = in_file.metadata().expect("couldn't read metadata").len();

    let target_crc = u32::from_str_radix(&args[2].to_str().expect("bad checksum string"), 16)
        .expect("bad checksum string");

    let chunk_size = usize::from_str_radix(&args[3].to_str().expect("bad chunk size string"), 16)
        .expect("bad chunk size string");

    assert!(in_len as usize as u64 == in_len);
    let in_len = in_len as usize;

    let mut input = Vec::with_capacity(in_len);
    in_file.read_to_end(&mut input).expect("couldn't read file");
    assert!(input.len() == in_len);
    // remove mutability
    let input = input;

    let whole_chunk_count = in_len / chunk_size;

    let zero_chunk = vec![0; chunk_size];
    let mut zeroes_hasher = Hasher::new();
    let mut zeroes_len = in_len - whole_chunk_count * chunk_size;
    zeroes_hasher.update(&zero_chunk[..zeroes_len]);

    for chunk_index in (0..=whole_chunk_count).rev() {
        let i = chunk_index * chunk_size;

        assert_eq!(zeroes_len + i, in_len);
        let mut hasher = Hasher::new();
        hasher.update(&input[0..i]);
        hasher.combine(&zeroes_hasher);
        let crc = hasher.finalize();

        if crc == target_crc {
            println!("zeroed from {:x}", i);
        }

        zeroes_hasher.update(&zero_chunk);
        zeroes_len += zero_chunk.len();
    }
}
