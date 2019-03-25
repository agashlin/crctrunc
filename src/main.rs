//! Check if a file could be truncated with zeroes to match the given `target_checksum`.
//! Bytes are kept from the start of the file in chunks of `chunk_size`.

extern crate crc32fast;

use std::cmp;
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

    let zeroes = vec![0; chunk_size];
    let whole_chunk_count = in_len / chunk_size;

    let mut input = Vec::with_capacity(in_len);
    in_file.read_to_end(&mut input).expect("couldn't read file");
    assert!(input.len() == in_len);
    // remove mutability
    let input = input;

    // Compute partial hashes going forward
    let hasher = Hasher::new();
    // `hashes[i]` contains the crc after hashing chunk #i
    let hashes: Vec<u32> = (0..whole_chunk_count)
        .scan(hasher, |hasher, i| {
            let chunk_start = i * chunk_size;
            let chunk_end = chunk_start + chunk_size;
            if chunk_end > in_len {
                return None;
            }
            hasher.update(&input[chunk_start..chunk_end]);
            Some(hasher.clone().finalize())
        })
        .collect();

    // Build up zeroes going backward
    // Initialize with the last partial chunk
    let mut hasher = Hasher::new();
    hasher.update(&zeroes[..(in_len - whole_chunk_count * chunk_size)]);

    for i in (0..whole_chunk_count).rev() {
        // Everything past chunk #i is zero
        let mut current = Hasher::new_with_initial(hashes[i]);
        current.combine(&hasher);

        if current.finalize() == target_crc {
            println!("zeroed from {:x}", (i + 1) * chunk_size);
        }

        hasher.update(&zeroes);
    }
}
