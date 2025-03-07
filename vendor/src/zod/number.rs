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
  // 整数かどうかの制約
  is_int: bool,
  // 倍数の制約（multipleOf）
  multiple_of: Option<f64>,
  // 有限数かどうかの制約
  is_finite: bool,
  // 安全な整数範囲内かどうかの制約
  is_safe: bool,
}

#[wasm_bindgen]
impl ZodNumber {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodNumber {
      base: ZodTypeBase::new("number"),
      min: None,
      gt: false,
      max: None,
      lt: false,
      is_int: false,
      multiple_of: None,
      is_finite: false,
      is_safe: false,
    }
  }

  // 「より大きい」(>)の検証メソッド
  #[wasm_bindgen]
  pub fn gt(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(value),
      gt: true,
      max: self.max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }

  // 「以上」(>=)の検証メソッド
  #[wasm_bindgen]
  pub fn gte(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(value),
      gt: false,
      max: self.max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }

  // 「未満」(<)の検証メソッド
  #[wasm_bindgen]
  pub fn lt(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: Some(value),
      lt: true,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }

  // 「以下」(<=)の検証メソッド
  #[wasm_bindgen]
  pub fn lte(&self, value: f64) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: Some(value),
      lt: false,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }
  
  // 整数かどうかを検証するメソッド
  #[wasm_bindgen]
  pub fn int(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: self.max,
      lt: self.lt,
      is_int: true,
      multiple_of: self.multiple_of,
      is_finite: true, // 整数は常に有限数
      is_safe: self.is_safe,
    }
  }
  
  // 正の数（0より大きい）を検証するメソッド
  #[wasm_bindgen]
  pub fn positive(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(0.0),
      gt: true,
      max: self.max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }
  
  // 非負の数（0以上）を検証するメソッド
  #[wasm_bindgen]
  pub fn nonnegative(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(0.0),
      gt: false,
      max: self.max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }
  
  // 負の数（0より小さい）を検証するメソッド
  #[wasm_bindgen]
  pub fn negative(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: Some(0.0),
      lt: true,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }
  
  // 非正の数（0以下）を検証するメソッド
  #[wasm_bindgen]
  pub fn nonpositive(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: Some(0.0),
      lt: false,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: self.is_finite,
      is_safe: self.is_safe,
    }
  }
  
  // 指定された値の倍数であることを検証するメソッド
  #[wasm_bindgen(js_name = multipleOf)]
  pub fn multiple_of(&self, value: f64) -> ZodNumber {
    // 整数の倍数を指定した場合は、結果も整数となる
    let is_int = if value.fract() == 0.0 { true } else { self.is_int };
    
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: self.max,
      lt: self.lt,
      is_int,
      multiple_of: Some(value),
      is_finite: true, // 倍数の設定は常に有限数を対象とする
      is_safe: self.is_safe,
    }
  }
  
  // 有限数のみを許可する検証メソッド
  #[wasm_bindgen]
  pub fn finite(&self) -> ZodNumber {
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      gt: self.gt,
      max: self.max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: true,
      is_safe: self.is_safe,
    }
  }
  
  // 安全な整数範囲内の値のみを許可する検証メソッド
  #[wasm_bindgen]
  pub fn safe(&self) -> ZodNumber {
    const MIN_SAFE_INT: f64 = -9007199254740991.0; // Number.MIN_SAFE_INTEGER
    const MAX_SAFE_INT: f64 = 9007199254740991.0;  // Number.MAX_SAFE_INTEGER
    
    // 既存の範囲制約と安全な整数範囲を組み合わせる
    let min = match self.min {
      Some(value) => Some(value.max(MIN_SAFE_INT)),
      None => Some(MIN_SAFE_INT),
    };
    
    let max = match self.max {
      Some(value) => Some(value.min(MAX_SAFE_INT)),
      None => Some(MAX_SAFE_INT),
    };
    
    ZodNumber {
      base: ZodTypeBase::new(&self.base.type_name),
      min,
      gt: self.gt,
      max,
      lt: self.lt,
      is_int: self.is_int,
      multiple_of: self.multiple_of,
      is_finite: true, // 安全な整数範囲は有限数のみ
      is_safe: true,
    }
  }
  
  // step (multipleOfのエイリアス)
  #[wasm_bindgen]
  pub fn step(&self, value: f64) -> ZodNumber {
    self.multiple_of(value)
  }

  // min (gteのエイリアス) - 最小値と最大値の両方が設定されていれば有限数
  #[wasm_bindgen()]
  pub fn min(&self, value: f64) -> ZodNumber {
    let mut result = self.gte(value);
    
    // 最大値も設定されている場合は、有限数として扱う
    if result.max.is_some() {
      result.is_finite = true;
    }
    
    result
  }
  
  // max (lteのエイリアス) - 最小値と最大値の両方が設定されていれば有限数
  #[wasm_bindgen()]
  pub fn max(&self, value: f64) -> ZodNumber {
    let mut result = self.lte(value);
    
    // 最小値も設定されている場合は、有限数として扱う
    if result.min.is_some() {
      result.is_finite = true;
    }
    
    result
  }

  // is_finiteフィールドのゲッター
  #[wasm_bindgen(getter, js_name = isFinite)]
  pub fn is_finite(&self) -> bool {
    self.is_finite
  }

  // is_intフィールドのゲッター
  #[wasm_bindgen(getter, js_name = isInt)]
  pub fn is_int(&self) -> bool {
    self.is_int
  }

  // is_safeフィールドのゲッター
  #[wasm_bindgen(getter, js_name = isSafe)]
  pub fn is_safe(&self) -> bool {
    self.is_safe
  }

  // min値のゲッター
  #[wasm_bindgen(getter, js_name = minValue)]
  pub fn min_value(&self) -> JsValue {
    match self.min {
      Some(value) => JsValue::from_f64(value),
      None => JsValue::null(),
    }
  }

  // max値のゲッター
  #[wasm_bindgen(getter, js_name = maxValue)]
  pub fn max_value(&self) -> JsValue {
    match self.max {
      Some(value) => JsValue::from_f64(value),
      None => JsValue::null(),
    }
  }

  // 内部処理用のメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    // 値がNaNかどうかをチェック
    if value.as_f64().map_or(false, |n| n.is_nan()) {
      return super::types::create_result_object(
        "error",
        &JsValue::from_str("Expected number, received NaN")
      );
    }
    
    // 基本的な型チェック
    let base_result = <Self as ZodType>::_create_parse_result(self, value);
    let status = js_sys::Reflect::get(&base_result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "error" {
      return base_result;
    }
    
    // 値が数値であることが確認できたら、制約をチェック
    if let Some(num) = value.as_f64() {
      // 有限数制約のチェック
      if self.is_finite && (num.is_infinite() || num.is_nan()) {
        return super::types::create_result_object(
          "error",
          &JsValue::from_str("Number must be finite, not Infinity or NaN")
        );
      }
      
      // 無限大でない場合は追加のチェックを行う
      if !num.is_infinite() {
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
        
        // 整数制約のチェック
        if self.is_int && num.fract() != 0.0 {
          return super::types::create_result_object(
            "error",
            &JsValue::from_str("Number must be an integer")
          );
        }
        
        // 倍数制約のチェック
        if let Some(multiple) = self.multiple_of {
          if multiple != 0.0 {
            // 浮動小数点の除算で精度の問題が発生するため
            // 四捨五入した比率が整数かどうかをチェックする方法に変更
            let ratio = num / multiple;
            let rounded_ratio = ratio.round();
            
            // 比率が整数に近い（差がごくわずか）ならOK
            if (ratio - rounded_ratio).abs() > 1e-10 {
              return super::types::create_result_object(
                "error",
                &JsValue::from_str(&format!("Number must be a multiple of {}", multiple))
              );
            }
          }
        }
        
        // 安全な整数範囲の制約チェック
        if self.is_safe {
          const MIN_SAFE_INT: f64 = -9007199254740991.0; // Number.MIN_SAFE_INTEGER
          const MAX_SAFE_INT: f64 = 9007199254740991.0;  // Number.MAX_SAFE_INTEGER
          
          if num < MIN_SAFE_INT || num > MAX_SAFE_INT {
            return super::types::create_result_object(
              "error",
              &JsValue::from_str("Number must be between -2^53+1 and 2^53-1")
            );
          }
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
