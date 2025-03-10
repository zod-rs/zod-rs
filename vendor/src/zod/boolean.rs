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

  // 内部実装用のパースメソッド
  fn _parse_internal(&self, value: &JsValue) -> JsValue {
    // 基本的な型チェック
    let base_result = <Self as ZodType>::_create_parse_result(self, value);
    let status = js_sys::Reflect::get(&base_result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "error" {
      return base_result;
    }
    
    // すべての検証をパスしたら成功
    super::types::create_result_object("ok", value)
  }
  
}

// ZodBoolean型にZodTypeトレイトを実装
impl ZodType for ZodBoolean {
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
impl_js_methods!(ZodBoolean);
