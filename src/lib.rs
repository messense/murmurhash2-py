use byteorder::{ByteOrder, LittleEndian};
use pyo3::prelude::*;

const M: u32 = 0x5bd1_e995;

// Code taken from https://github.com/tantivy-search/murmurhash32
fn murmurhash2_impl(key: &[u8], seed: u32) -> u32 {
    let mut h: u32 = seed ^ (key.len() as u32);

    let mut four_bytes_chunks = key.chunks_exact(4);

    while let Some(chunk) = four_bytes_chunks.next() {
        let mut k: u32 = LittleEndian::read_u32(chunk);
        k = k.wrapping_mul(M);
        k ^= k >> 24;
        k = k.wrapping_mul(M);
        h = h.wrapping_mul(M);
        h ^= k;
    }
    let remainder = four_bytes_chunks.remainder();

    // Handle the last few bytes of the input array
    match remainder.len() {
        3 => {
            h ^= u32::from(remainder[2]) << 16;
            h ^= u32::from(remainder[1]) << 8;
            h ^= u32::from(remainder[0]);
            h = h.wrapping_mul(M);
        }
        2 => {
            h ^= u32::from(remainder[1]) << 8;
            h ^= u32::from(remainder[0]);
            h = h.wrapping_mul(M);
        }
        1 => {
            h ^= u32::from(remainder[0]);
            h = h.wrapping_mul(M);
        }
        _ => {}
    }
    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^ (h >> 15)
}

#[pymodule]
fn murmurhash2(_py: Python, m: &PyModule) -> PyResult<()> {
    /// murmurhash2 hash function
    ///
    /// Arguments
    /// key: data to be hashed
    /// seed: hash seed
    #[pyfn(m, "murmurhash2")]
    fn do_murmurhash2(py: Python, key: &[u8], seed: u32) -> u32 {
        py.allow_threads(|| murmurhash2_impl(key, seed))
    }

    Ok(())
}
