use wasm_bindgen::prelude::*;

// JavaScriptオブジェクトに変換するためのヘルパー関数
pub fn create_result_object(status: &str, value: &JsValue) -> JsValue {
  let obj = js_sys::Object::new();
  js_sys::Reflect::set(&obj, &JsValue::from_str("status"), &JsValue::from_str(status)).unwrap();
  js_sys::Reflect::set(&obj, &JsValue::from_str("value"), value).unwrap();
  obj.into()
}

// 型情報を保持する基本構造体
#[wasm_bindgen]
pub struct ZodTypeBase {
  // 型名を保持するフィールド
  #[wasm_bindgen(skip)]
  pub type_name: String,
}

// ZodTypeトレイト - すべてのZod型が実装する必要があるインターフェース
pub trait ZodType {
  // 型名を返すメソッド
  fn r#type(&self) -> &str;
  
  // 値の型を判定するヘルパーメソッド
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
  
  // 型チェックを行う共通メソッド
  fn _check_type(&self, value: &JsValue) -> bool {
    self._get_type(value) == self.r#type()
  }
  
  // パース結果を生成する共通メソッド
  fn _create_parse_result(&self, value: &JsValue) -> JsValue {
    if self._check_type(value) {
      create_result_object("ok", value)
    } else {
      create_result_object("error", value)
    }
  }
}

// ZodTypeBaseにZodTypeトレイトを実装
impl ZodType for ZodTypeBase {
  fn r#type(&self) -> &str {
    &self.type_name
  }
}
