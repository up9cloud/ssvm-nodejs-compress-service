use wasm_bindgen::prelude::*;

use lzma_rs::{lzma_compress, lzma_decompress};
use std::io::Cursor;

#[wasm_bindgen(catch)]
pub fn compress(src: &mut [u8], _preset: u32) -> Result<Vec<u8>, JsValue> {
  let mut src_buf = Cursor::new(src);
  let mut buf: Vec<u8> = vec![];
  match lzma_compress(&mut src_buf, &mut buf) {
    Ok(_v) => Ok(buf),
    Err(e) => Err(JsValue::from(format!("{}", e))),
  }
}

#[wasm_bindgen(catch)]
pub fn decompress(src: &mut [u8]) -> Result<Vec<u8>, JsValue> {
  let mut src_buf = Cursor::new(src);
  let mut buf: Vec<u8> = vec![];
  match lzma_decompress(&mut src_buf, &mut buf) {
    Ok(_v) => Ok(buf),
    Err(e) => Err(JsValue::from(format!("{:?}", e))),
  }
}

// It seems wasm_bindgen not support (catch) yet...?
