use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn int_add(a: u32, b: u32) -> u32 {
  a + b
}

// Not work!
#[wasm_bindgen]
pub fn int_multi_option(a: u32, b: u32) -> Option<u32> {
  if a == 0 {
    None
  } else {
    Some(a * b)
  }
}

// Not work! (instantiation failed)
#[wasm_bindgen]
pub fn int_multi_result(a: u32, b: u32) -> Result<u32, JsValue> {
  if a == 0 {
    Err(JsValue::from("error"))
  } else {
    Ok(a * b)
  }
}

#[wasm_bindgen]
pub fn str_concat(a: &[u8], b: &[u8]) -> String {
  match String::from_utf8([a, b].concat()) {
    Ok(v) => v,
    Err(_) => String::from("")
  }
}
