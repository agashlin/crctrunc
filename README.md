# crctrunc
Check if a file could be truncated with zeroes to match a CRC32 checksum.

## Usage

`crctrunc file target_checksum chunk_size`

Bytes are kept from the start of the file in chunks of `chunk_size`. All integers are hexadecimal.
`chunk_size` can be set to `1` to try all possible truncations, but this can be slow. Usually something like 1K (`400`) or 64K (`10000`) is more appropriate.

## Example
`crctrunc omni.ja 326fbb3c 10000`

This will check the CRC with bytes from offset `10000` to the end of the file zeroed, then from `20000`, `30000`, etc. (though matching is actually done backwards).
