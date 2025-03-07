use wasm_bindgen::prelude::*;

pub struct ParseReturnType<V> {
  pub status: String,
  pub value: V,
}

trait ZodType {
  fn r#type(&self) -> &str;
  fn _get_type(&self, value: &JsValue) -> String;
}

pub struct ZodNumber {
  r#type: String,
}

impl ZodNumber {
  pub fn new() -> Self {
    ZodNumber {
      r#type: "number".to_string(),
    }
  }

  // 直接ZodNumberにメソッドとして実装
  pub fn _parse(&self, value: &JsValue) -> ParseReturnType<JsValue> {
    let parsed_type = self._get_type(value);
    if parsed_type != self.r#type {
      return ParseReturnType {
        status: "error".to_string(),
        value: value.clone(),
      };
    }
    
    match value.as_f64() {
      Some(_) => ParseReturnType {
        status: "ok".to_string(),
        value: value.clone(),
      },
      None => ParseReturnType {
        status: "error".to_string(),
        value: value.clone(),
      }
    }
  }
}

impl ZodType for ZodNumber {
  fn r#type(&self) -> &str {
    &self.r#type
  }
  
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

