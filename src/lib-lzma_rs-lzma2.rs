use wasm_bindgen::prelude::*;

use lzma_rs::{lzma2_compress, lzma2_decompress};
use std::io::Cursor;

#[wasm_bindgen]
pub fn compress(src: &[u8], _preset: u32) -> Vec<u8> {
  let mut src_buf = Cursor::new(src);
  let mut buf: Vec<u8> = vec![];
  match lzma2_compress(&mut src_buf, &mut buf) {
    Ok(_v) => buf,
    Err(_) => vec![],
  }
}

#[wasm_bindgen]
pub fn decompress(src: &[u8]) -> Vec<u8> {
  let mut src_buf = Cursor::new(src);
  let mut buf: Vec<u8> = vec![];
  match lzma2_decompress(&mut src_buf, &mut buf) {
    Ok(_v) => buf,
    Err(_) => vec![],
  }
}

// Output file not as expectation...
