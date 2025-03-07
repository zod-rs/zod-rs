use wasm_bindgen::prelude::*;
use super::types::{ZodType, create_result_object};

#[wasm_bindgen]
pub struct ZodString {
  r#type: String,
}

#[wasm_bindgen]
impl ZodString {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodString {
      r#type: "string".to_string(),
    }
  }

  // 内部処理用のメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    let parsed_type = <Self as ZodType>::_get_type(self, &value);
    if parsed_type != self.r#type {
      return create_result_object("error", &value);
    }
    
    match value.as_string() {
      Some(_) => create_result_object("ok", value),
      None => create_result_object("error", value)
    }
  }
  
  // JavaScriptから利用可能な公開メソッド
  #[wasm_bindgen]
  pub fn parse(&self, value: JsValue) -> JsValue {
    let result = self._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      return value;
    } else {
      // エラーをスロー
      let error_msg = format!("Expected string, received {}", <Self as ZodType>::_get_type(self, &value));
      wasm_bindgen::throw_str(&error_msg);
      // throwの後は実行されないが、コンパイラのために値を返す
      JsValue::NULL
    }
  }
}

impl ZodType for ZodString {
  fn r#type(&self) -> &str {
    &self.r#type
  }
}
