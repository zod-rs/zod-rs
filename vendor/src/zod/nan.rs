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

  // 内部実装用のパースメソッド
  fn _parse_internal(&self, value: &JsValue) -> JsValue {
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
  
  // トレイト要件の_parseメソッド実装
  fn _parse(&self, value: &JsValue) -> JsValue {
    self._parse_internal(value)
  }
}

// JavaScriptインターフェース用のメソッドを実装
use crate::impl_js_methods;
impl_js_methods!(ZodNaN);
