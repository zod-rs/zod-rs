use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};

#[wasm_bindgen]
pub struct ZodNaN {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
}

#[wasm_bindgen]
impl ZodNaN {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodNaN {
      base: ZodTypeBase::new("nan"),
    }
  }

  // 内部処理用のパースメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    // 値がNaNかどうかをチェック
    if self._is_nan(value) {
      // NaNである場合は成功
      super::types::create_result_object("ok", value)
    } else {
      // NaNでない場合はエラー
      super::types::create_result_object(
        "error",
        &JsValue::from_str(&format!("Expected NaN, received {}", <Self as ZodType>::_get_type(self, value)))
      )
    }
  }
  
  // JavaScriptから利用可能な公開メソッド
  #[wasm_bindgen]
  pub fn parse(&self, value: JsValue) -> JsValue {
    let result = self._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      // 成功した場合、値をそのまま返す
      return value;
    } else {
      // エラーメッセージを取得
      let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      let error_msg = if error_value.is_string() {
        error_value.as_string().unwrap()
      } else {
        format!("Expected NaN, received {}", <Self as ZodType>::_get_type(self, &value))
      };
      
      wasm_bindgen::throw_str(&error_msg);
    }
  }
  
  // 値がNaNかどうかを判定するヘルパーメソッド
  fn _is_nan(&self, value: &JsValue) -> bool {
    let is_nan_func = js_sys::Function::new_with_args(
      "val", 
      "return Number.isNaN(val);"
    );
    
    is_nan_func.call1(&JsValue::NULL, value)
      .unwrap()
      .as_bool()
      .unwrap_or(false)
  }
}

// ZodNaN型にZodTypeトレイトを実装
impl ZodType for ZodNaN {
  fn r#type(&self) -> &str {
    &self.base.type_name
  }
}
