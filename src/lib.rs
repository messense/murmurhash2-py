use byteorder::{ByteOrder, LittleEndian};
use pyo3::prelude::*;

// Code taken from https://github.com/tantivy-search/murmurhash32

// murmurhash2 32bit
const M: u32 = 0x5bd1_e995;

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

// murmurhash3 32bit
const C1: u32 = 0xcc9e_2d51;
const C2: u32 = 0x1b87_3593;
const D: u32 = 0xe654_6b64;
const FMIX1: u32 = 0x85eb_ca6b;
const FMIX2: u32 = 0xc2b2_ae35;

#[inline(always)]
fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(FMIX1);
    h ^= h >> 13;
    h = h.wrapping_mul(FMIX2);
    h ^= h >> 16;
    h
}

pub fn murmurhash3_impl(key: &[u8], seed: u32) -> u32 {
    let mut h: u32 = seed;

    let mut four_bytes_chunks = key.chunks_exact(4);

    while let Some(chunk) = four_bytes_chunks.next() {
        let mut k: u32 = LittleEndian::read_u32(chunk);
        k = k.wrapping_mul(C1);
        k = k.rotate_left(15);
        k = k.wrapping_mul(C2);
        h ^= k;
        h = h.rotate_left(13);
        h = (h.wrapping_mul(5)).wrapping_add(D);
    }

    let remainder = four_bytes_chunks.remainder();
    match remainder.len() {
        3 => {
            let mut k = u32::from(remainder[2]) << 16;
            k ^= u32::from(remainder[1]) << 8;
            k ^= u32::from(remainder[0]);
            k = k.wrapping_mul(C1);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2);
            h ^= k;
        }
        2 => {
            let mut k = u32::from(remainder[1]) << 8;
            k ^= u32::from(remainder[0]);
            k = k.wrapping_mul(C1);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2);
            h ^= k;
        }
        1 => {
            let mut k = u32::from(remainder[0]);
            k = k.wrapping_mul(C1);
            k = k.rotate_left(15);
            k = k.wrapping_mul(C2);
            h ^= k;
        }
        _ => {}
    }
    fmix32(h ^ key.len() as u32)
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

    /// murmurhash3 hash function
    ///
    /// Arguments
    /// key: data to be hashed
    /// seed: hash seed
    #[pyfn(m, "murmurhash3")]
    fn do_murmurhash3(py: Python, key: &[u8], seed: u32) -> u32 {
        py.allow_threads(|| murmurhash3_impl(key, seed))
    }

    Ok(())
}
