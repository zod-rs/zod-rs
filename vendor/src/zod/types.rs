use wasm_bindgen::prelude::*;

pub struct ParseReturnType<V> {
  pub status: String,
  pub value: V,
}

// JavaScriptオブジェクトに変換するためのヘルパー関数
pub fn create_result_object(status: &str, value: &JsValue) -> JsValue {
  let obj = js_sys::Object::new();
  js_sys::Reflect::set(&obj, &JsValue::from_str("status"), &JsValue::from_str(status)).unwrap();
  js_sys::Reflect::set(&obj, &JsValue::from_str("value"), value).unwrap();
  obj.into()
}

pub trait ZodType {
  fn r#type(&self) -> &str;
  fn _get_type(&self, value: &JsValue) -> String {
    if value.as_f64().is_some() {
      return "number".to_string();
    }
    if value.as_string().is_some() {
      return "string".to_string();
    }
    if value.as_bool().is_some() {
      return "boolean".to_string();
    }
    if value.is_null() {
      return "null".to_string();
    }
    if value.is_undefined() {
      return "undefined".to_string();
    }
    if value.is_object() {
      return "object".to_string();
    }
    if value.is_function() {
      return "function".to_string();
    }
    "unknown".to_string()
  }
}
