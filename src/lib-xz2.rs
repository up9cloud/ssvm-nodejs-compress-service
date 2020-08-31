use wasm_bindgen::prelude::*;

use std::io::prelude::*;
use xz2::read::{XzDecoder, XzEncoder};

#[wasm_bindgen(catch)]
pub fn compress(src_buf: &[u8], preset: u32) -> Result<Vec<u8>, JsValue> {
  let mut compressor = XzEncoder::new(src_buf, preset);
  let mut buf: Vec<u8> = vec![];
  match compressor.read_to_end(&mut buf) {
    Ok(v) => Ok(buf),
    Err(e) => Err(format!("{}", e)),
  }
}

#[wasm_bindgen(catch)]
pub fn decompress(src_buf: &[u8]) -> Result<Vec<u8>, JsValue> {
  let mut decompressor = XzDecoder::new(src_buf);
  let mut buf: Vec<u8> = vec![];
  match decompressor.read_to_end(&mut buf) {
    Ok(v) => Ok(buf),
    Err(e) => Err(format!("{}", e)),
  }
}

// Can't compile :/
