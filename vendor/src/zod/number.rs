use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};

#[wasm_bindgen]
pub struct ZodNumber {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
  // 最小値の制約（gt/gte）
  min: Option<f64>,
  // gt=trueの場合は「より大きい」、gt=falseの場合は「以上」
  gt: bool,
  // 最大値の制約（lt/lte）
  max: Option<f64>,
  // lt=trueの場合は「未満」、lt=falseの場合は「以下」
  lt: bool,
}

#[wasm_bindgen]
impl ZodNumber {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodNumber {
      base: ZodTypeBase {
        type_name: "number".to_string(),
      },
      min: None,
      gt: false,
      max: None,
      lt: false,
    }
  }

  // 「より大きい」(>)の検証メソッド
  #[wasm_bindgen]
  pub fn gt(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase {
        type_name: self.base.type_name.clone(),
      },
      min: Some(value),
      gt: true,
      max: self.max,
      lt: self.lt,
    }
  }

  // 「以上」(>=)の検証メソッド
  #[wasm_bindgen]
  pub fn gte(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase {
        type_name: self.base.type_name.clone(),
      },
      min: Some(value),
      gt: false,
      max: self.max,
      lt: self.lt,
    }
  }

  // 「未満」(<)の検証メソッド
  #[wasm_bindgen]
  pub fn lt(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase {
        type_name: self.base.type_name.clone(),
      },
      min: self.min,
      gt: self.gt,
      max: Some(value),
      lt: true,
    }
  }

  // 「以下」(<=)の検証メソッド
  #[wasm_bindgen]
  pub fn lte(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase {
        type_name: self.base.type_name.clone(),
      },
      min: self.min,
      gt: self.gt,
      max: Some(value),
      lt: false,
    }
  }

  // 内部処理用のメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    // 基本的な型チェック
    let base_result = <Self as ZodType>::_create_parse_result(self, value);
    let status = js_sys::Reflect::get(&base_result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "error" {
      return base_result;
    }
    
    // 値が数値であることが確認できたら、制約をチェック
    if let Some(num) = value.as_f64() {
      // min/gtの制約チェック
      if let Some(min_value) = self.min {
        if self.gt && num <= min_value {
          return super::types::create_result_object(
            "error", 
            &JsValue::from_str(&format!("Number must be greater than {}", min_value))
          );
        } else if !self.gt && num < min_value {
          return super::types::create_result_object(
            "error", 
            &JsValue::from_str(&format!("Number must be greater than or equal to {}", min_value))
          );
        }
      }
      
      // max/ltの制約チェック
      if let Some(max_value) = self.max {
        if self.lt && num >= max_value {
          return super::types::create_result_object(
            "error", 
            &JsValue::from_str(&format!("Number must be less than {}", max_value))
          );
        } else if !self.lt && num > max_value {
          return super::types::create_result_object(
            "error", 
            &JsValue::from_str(&format!("Number must be less than or equal to {}", max_value))
          );
        }
      }
    }
    
    // すべての検証をパスしたら成功
    super::types::create_result_object("ok", value)
  }
  
  // JavaScriptから利用可能な公開メソッド
  // 戻り値をJsValueではなく明示的に数値として指定
  #[wasm_bindgen]
  pub fn parse(&self, value: JsValue) -> f64 {
    let result = self._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      // 値が数値の場合、そのまま返す
      if let Some(num) = value.as_f64() {
        return num;
      } else {
        // 通常はここに到達しないはず
        return 0.0;
      }
    } else {
      // エラーメッセージを取得
      let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      let error_msg = if error_value.is_string() {
        error_value.as_string().unwrap()
      } else {
        format!("Expected number, received {}", <Self as ZodType>::_get_type(self, &value))
      };
      
      wasm_bindgen::throw_str(&error_msg);
    }
  }
}

// ZodNumber型にZodTypeトレイトを実装
impl ZodType for ZodNumber {
  fn r#type(&self) -> &str {
    &self.base.type_name
  }
}
