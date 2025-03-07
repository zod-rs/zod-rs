use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};

#[wasm_bindgen]
pub struct ZodString {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
}

#[wasm_bindgen]
impl ZodString {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodString {
      base: ZodTypeBase {
        type_name: "string".to_string(),
      },
    }
  }

  // 内部処理用のメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    <Self as ZodType>::_create_parse_result(self, value)
  }
  
  // JavaScriptから利用可能な公開メソッド
  // 戻り値をJsValueではなく明示的に文字列として指定
  #[wasm_bindgen]
  pub fn parse(&self, value: JsValue) -> String {
    let result = self._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      // 値が文字列の場合はそのまま返す
      if let Some(str_val) = value.as_string() {
        return str_val;
      } else {
        // 通常はここに到達しないはず
        return "".to_string();
      }
    } else {
      // エラーをスロー
      let error_msg = format!("Expected string, received {}", <Self as ZodType>::_get_type(self, &value));
      wasm_bindgen::throw_str(&error_msg);
    }
  }
}

// ZodString型にZodTypeトレイトを実装
impl ZodType for ZodString {
  fn r#type(&self) -> &str {
    &self.base.type_name
  }
}
