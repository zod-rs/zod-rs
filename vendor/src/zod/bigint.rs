use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};

#[wasm_bindgen]
pub struct ZodBigInt {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
  // 最小値の制約（gt/gte）
  min: Option<String>,
  // gt=trueの場合は「より大きい」、gt=falseの場合は「以上」
  gt: bool,
  // 最大値の制約（lt/lte）
  max: Option<String>,
  // lt=trueの場合は「未満」、lt=falseの場合は「以下」
  lt: bool,
  // 倍数の制約（multipleOf）
  multiple_of: Option<String>,
  // 正の数であることの制約
  is_positive: bool,
  // 非負の数であることの制約
  is_nonnegative: bool,
  // 負の数であることの制約
  is_negative: bool,
  // 非正の数であることの制約
  is_nonpositive: bool,
}

#[wasm_bindgen]
impl ZodBigInt {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodBigInt {
      base: ZodTypeBase::new("bigint"),
      min: None,
      gt: false,
      max: None,
      lt: false,
      multiple_of: None,
      is_positive: false,
      is_nonnegative: false,
      is_negative: false,
      is_nonpositive: false,
    }
  }

  // 「より大きい」(>)の検証メソッド
  #[wasm_bindgen]
  pub fn gt(&self, value: JsValue) -> ZodBigInt {
    let bigint_str = if value.is_bigint() {
      // BigIntからJavaScriptのstringへ変換（Rustでは直接BigIntを扱えないため）
      let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
      big_int_to_string.call1(&JsValue::NULL, &value).unwrap().as_string().unwrap()
    } else {
      // 数値が渡された場合も処理（暗黙的にBigIntに変換）
      value.as_f64().map_or_else(
        || value.as_string().unwrap_or_else(|| "0".to_string()),
        |n| n.to_string()
      )
    };

    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(bigint_str),
      gt: true,
      max: self.max.clone(),
      lt: self.lt,
      multiple_of: self.multiple_of.clone(),
      is_positive: self.is_positive,
      is_nonnegative: self.is_nonnegative,
      is_negative: self.is_negative,
      is_nonpositive: self.is_nonpositive,
    }
  }

  // 「以上」(>=)の検証メソッド
  #[wasm_bindgen]
  pub fn gte(&self, value: JsValue) -> ZodBigInt {
    let bigint_str = if value.is_bigint() {
      let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
      big_int_to_string.call1(&JsValue::NULL, &value).unwrap().as_string().unwrap()
    } else {
      value.as_f64().map_or_else(
        || value.as_string().unwrap_or_else(|| "0".to_string()),
        |n| n.to_string()
      )
    };

    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(bigint_str),
      gt: false,
      max: self.max.clone(),
      lt: self.lt,
      multiple_of: self.multiple_of.clone(),
      is_positive: self.is_positive,
      is_nonnegative: self.is_nonnegative,
      is_negative: self.is_negative,
      is_nonpositive: self.is_nonpositive,
    }
  }

  // 「未満」(<)の検証メソッド
  #[wasm_bindgen]
  pub fn lt(&self, value: JsValue) -> ZodBigInt {
    let bigint_str = if value.is_bigint() {
      let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
      big_int_to_string.call1(&JsValue::NULL, &value).unwrap().as_string().unwrap()
    } else {
      value.as_f64().map_or_else(
        || value.as_string().unwrap_or_else(|| "0".to_string()),
        |n| n.to_string()
      )
    };

    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min.clone(),
      gt: self.gt,
      max: Some(bigint_str),
      lt: true,
      multiple_of: self.multiple_of.clone(),
      is_positive: self.is_positive,
      is_nonnegative: self.is_nonnegative,
      is_negative: self.is_negative,
      is_nonpositive: self.is_nonpositive,
    }
  }

  // 「以下」(<=)の検証メソッド
  #[wasm_bindgen]
  pub fn lte(&self, value: JsValue) -> ZodBigInt {
    let bigint_str = if value.is_bigint() {
      let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
      big_int_to_string.call1(&JsValue::NULL, &value).unwrap().as_string().unwrap()
    } else {
      value.as_f64().map_or_else(
        || value.as_string().unwrap_or_else(|| "0".to_string()),
        |n| n.to_string()
      )
    };

    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min.clone(),
      gt: self.gt,
      max: Some(bigint_str),
      lt: false,
      multiple_of: self.multiple_of.clone(),
      is_positive: self.is_positive,
      is_nonnegative: self.is_nonnegative,
      is_negative: self.is_negative,
      is_nonpositive: self.is_nonpositive,
    }
  }

  // 正の数（0より大きい）を検証するメソッド
  #[wasm_bindgen]
  pub fn positive(&self) -> ZodBigInt {
    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some("0".to_string()),
      gt: true,
      max: self.max.clone(),
      lt: self.lt,
      multiple_of: self.multiple_of.clone(),
      is_positive: true,
      is_nonnegative: false,
      is_negative: false,
      is_nonpositive: false,
    }
  }
  
  // 非負の数（0以上）を検証するメソッド
  #[wasm_bindgen]
  pub fn nonnegative(&self) -> ZodBigInt {
    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some("0".to_string()),
      gt: false,
      max: self.max.clone(),
      lt: self.lt,
      multiple_of: self.multiple_of.clone(),
      is_positive: false,
      is_nonnegative: true,
      is_negative: false,
      is_nonpositive: false,
    }
  }
  
  // 負の数（0より小さい）を検証するメソッド
  #[wasm_bindgen]
  pub fn negative(&self) -> ZodBigInt {
    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min.clone(),
      gt: self.gt,
      max: Some("0".to_string()),
      lt: true,
      multiple_of: self.multiple_of.clone(),
      is_positive: false,
      is_nonnegative: false,
      is_negative: true,
      is_nonpositive: false,
    }
  }
  
  // 非正の数（0以下）を検証するメソッド
  #[wasm_bindgen]
  pub fn nonpositive(&self) -> ZodBigInt {
    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min.clone(),
      gt: self.gt,
      max: Some("0".to_string()),
      lt: false,
      multiple_of: self.multiple_of.clone(),
      is_positive: false,
      is_nonnegative: false,
      is_negative: false,
      is_nonpositive: true,
    }
  }
  
  // 指定された値の倍数であることを検証するメソッド
  #[wasm_bindgen(js_name = multipleOf)]
  pub fn multiple_of(&self, value: JsValue) -> ZodBigInt {
    let bigint_str = if value.is_bigint() {
      let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
      big_int_to_string.call1(&JsValue::NULL, &value).unwrap().as_string().unwrap()
    } else {
      value.as_f64().map_or_else(
        || value.as_string().unwrap_or_else(|| "0".to_string()),
        |n| n.to_string()
      )
    };

    ZodBigInt {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min.clone(),
      gt: self.gt,
      max: self.max.clone(),
      lt: self.lt,
      multiple_of: Some(bigint_str),
      is_positive: self.is_positive,
      is_nonnegative: self.is_nonnegative,
      is_negative: self.is_negative,
      is_nonpositive: self.is_nonpositive,
    }
  }

  // min (gteのエイリアス)
  #[wasm_bindgen]
  pub fn min(&self, value: JsValue) -> ZodBigInt {
    self.gte(value)
  }
  
  // max (lteのエイリアス)
  #[wasm_bindgen]
  pub fn max(&self, value: JsValue) -> ZodBigInt {
    self.lte(value)
  }
  
  // min値のゲッター
  #[wasm_bindgen(getter, js_name = minValue)]
  pub fn min_value(&self) -> JsValue {
    match &self.min {
      Some(value_str) => {
        // 文字列をBigIntに変換
        let to_bigint = js_sys::Function::new_with_args(
          "str", 
          "return BigInt(str);"
        );
        
        to_bigint.call1(&JsValue::NULL, &JsValue::from_str(value_str)).unwrap_or(JsValue::null())
      },
      None => JsValue::null(),
    }
  }

  // max値のゲッター
  #[wasm_bindgen(getter, js_name = maxValue)]
  pub fn max_value(&self) -> JsValue {
    match &self.max {
      Some(value_str) => {
        // 文字列をBigIntに変換
        let to_bigint = js_sys::Function::new_with_args(
          "str", 
          "return BigInt(str);"
        );
        
        to_bigint.call1(&JsValue::NULL, &JsValue::from_str(value_str)).unwrap_or(JsValue::null())
      },
      None => JsValue::null(),
    }
  }

  // 内部処理用のメソッド
  pub fn _parse(&self, value: &JsValue) -> JsValue {
    // 値がBigIntかどうかをチェック
    if !self._is_bigint(value) {
      return super::types::create_result_object(
        "error",
        &JsValue::from_str(&format!("Expected bigint, received {}", <Self as ZodType>::_get_type(self, value)))
      );
    }
    
    // BigIntをJavaScriptの文字列表現に変換
    let big_int_to_string = js_sys::Function::new_with_args("n", "return n.toString()");
    let value_str = big_int_to_string.call1(&JsValue::NULL, value).unwrap().as_string().unwrap();
    
    // 比較用の関数をJavaScriptで定義
    let compare_bigints = js_sys::Function::new_with_args(
      "a, b, operator", 
      r#"
      const bigA = BigInt(a);
      const bigB = BigInt(b);
      switch(operator) {
        case ">": return bigA > bigB;
        case ">=": return bigA >= bigB;
        case "<": return bigA < bigB;
        case "<=": return bigA <= bigB;
        case "===": return bigA === bigB;
        default: return false;
      }
      "#
    );
    
    // 制約のチェック
    // 最小値の制約
    if let Some(min) = &self.min {
      let op = if self.gt { ">" } else { ">=" };
      let result = compare_bigints.call3(
        &JsValue::NULL, 
        &JsValue::from_str(&value_str), 
        &JsValue::from_str(min),
        &JsValue::from_str(op)
      ).unwrap();
      
      if !result.as_bool().unwrap_or(false) {
        let error_msg = if self.gt {
          format!("BigInt must be greater than {}", min)
        } else {
          format!("BigInt must be greater than or equal to {}", min)
        };
        return super::types::create_result_object("error", &JsValue::from_str(&error_msg));
      }
    }
    
    // 最大値の制約
    if let Some(max) = &self.max {
      let op = if self.lt { "<" } else { "<=" };
      let result = compare_bigints.call3(
        &JsValue::NULL, 
        &JsValue::from_str(&value_str), 
        &JsValue::from_str(max),
        &JsValue::from_str(op)
      ).unwrap();
      
      if !result.as_bool().unwrap_or(false) {
        let error_msg = if self.lt {
          format!("BigInt must be less than {}", max)
        } else {
          format!("BigInt must be less than or equal to {}", max)
        };
        return super::types::create_result_object("error", &JsValue::from_str(&error_msg));
      }
    }
    
    // 倍数制約のチェック
    if let Some(multiple) = &self.multiple_of {
      let is_multiple_of = js_sys::Function::new_with_args(
        "value, multiple", 
        r#"
        if (multiple === "0") return true;
        const bigValue = BigInt(value);
        const bigMultiple = BigInt(multiple);
        return bigValue % bigMultiple === 0n;
        "#
      );
      
      let result = is_multiple_of.call2(
        &JsValue::NULL, 
        &JsValue::from_str(&value_str), 
        &JsValue::from_str(multiple)
      ).unwrap();
      
      if !result.as_bool().unwrap_or(false) {
        let error_msg = format!("BigInt must be a multiple of {}", multiple);
        return super::types::create_result_object("error", &JsValue::from_str(&error_msg));
      }
    }
    
    // すべての検証をパスしたら成功
    super::types::create_result_object("ok", value)
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
        format!("Expected bigint, received {}", <Self as ZodType>::_get_type(self, &value))
      };
      
      wasm_bindgen::throw_str(&error_msg);
    }
  }
  
  // 値がBigIntかどうかを判定するヘルパーメソッド
  fn _is_bigint(&self, value: &JsValue) -> bool {
    let is_bigint_func = js_sys::Function::new_with_args(
      "val", 
      "return typeof val === 'bigint';"
    );
    
    is_bigint_func.call1(&JsValue::NULL, value)
      .unwrap()
      .as_bool()
      .unwrap_or(false)
  }
}

// ZodBigInt型にZodTypeトレイトを実装
impl ZodType for ZodBigInt {
  fn r#type(&self) -> &str {
    &self.base.type_name
  }
  
  // BigInt型の判定を追加
  fn _get_type(&self, value: &JsValue) -> String {
    if self._is_bigint(value) {
      return "bigint".to_string();
    }
    
    // その他の型はデフォルト実装を使用
    <super::types::ZodTypeBase as ZodType>::_get_type(&self.base, value)
  }
}
