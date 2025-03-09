use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};

#[wasm_bindgen]
pub struct ZodBoolean {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
}

#[wasm_bindgen]
impl ZodBoolean {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodBoolean {
      base: ZodTypeBase::new("boolean"),
    }
  }

  // 内部処理用のパースメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    // 基本的な型チェック
    let base_result = <Self as ZodType>::_create_parse_result(self, value);
    let status = js_sys::Reflect::get(&base_result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "error" {
      return base_result;
    }
    
    // すべての検証をパスしたら成功
    super::types::create_result_object("ok", value)
  }
  
  // JavaScriptから利用可能な公開メソッド
  #[wasm_bindgen]
  pub fn parse(&self, value: JsValue) -> bool {
    let result = self._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      // 値がブール値の場合、そのまま返す
      if let Some(bool_value) = value.as_bool() {
        return bool_value;
      } else {
        // 通常はここに到達しないはず
        return false;
      }
    } else {
      // エラーメッセージを取得
      let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      let error_msg = if error_value.is_string() {
        error_value.as_string().unwrap()
      } else {
        format!("Expected boolean, received {}", <Self as ZodType>::_get_type(self, &value))
      };
      
      wasm_bindgen::throw_str(&error_msg);
    }
  }

  #[wasm_bindgen]
  pub fn safe_parse(&self, value: JsValue) -> Result<bool, String> {
      let result = self._parse(&value);
      let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
      if status.as_string().unwrap() == "ok" {
          if let Some(bool_value) = value.as_bool() {
              Ok(bool_value)
          } else {
              Err("Expected boolean, received non-boolean value".to_string())
          }
      } else {
          let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
          let error_msg = if error_value.is_string() {
              error_value.as_string().unwrap()
          } else {
              format!("Expected boolean, received {}", <Self as ZodType>::_get_type(self, &value))
          };
          Err(error_msg)
      }
  }
}

// ZodBoolean型にZodTypeトレイトを実装
impl ZodType for ZodBoolean {
  fn r#type(&self) -> &str {
    &self.base.type_name
  }
}
