use wasm_bindgen::prelude::*;
use super::types::{ZodType, ZodTypeBase};
use js_sys::RegExp;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct ZodString {
  // 基本的な型情報を持つ構造体
  base: ZodTypeBase,
  // 最小文字数の制約
  min: Option<usize>,
  // 最大文字数の制約
  max: Option<usize>,
  // 正確な文字数の制約
  length: Option<usize>,
  // Eメール形式のバリデーション
  is_email: bool,
  // URL形式のバリデーション
  is_url: bool,
  // 正規表現のバリデーション
  regex: Option<RegExp>,
  // 含むべき文字列
  includes: Option<String>,
  // includesの開始位置
  includes_position: Option<usize>,
  // 先頭一致する文字列
  starts_with: Option<String>,
  // 末尾一致する文字列
  ends_with: Option<String>,
  // 空文字列ではないことの制約のエラーメッセージ
  nonempty_message: Option<String>,
}

#[wasm_bindgen]
impl ZodString {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    ZodString {
      base: ZodTypeBase::new("string"),
      min: None,
      max: None,
      length: None,
      is_email: false,
      is_url: false,
      regex: None,
      includes: None,
      includes_position: None,
      starts_with: None,
      ends_with: None,
      nonempty_message: None,
    }
  }

  // 最小文字数を設定するメソッド
  #[wasm_bindgen]
  pub fn min(&self, value: usize) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(value),
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 最大文字数を設定するメソッド
  #[wasm_bindgen]
  pub fn max(&self, value: usize) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: Some(value),
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 正確な文字数を設定するメソッド
  #[wasm_bindgen]
  pub fn length(&self, value: usize) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(value),  // 長さ指定は最小と最大が同じ値
      max: Some(value),  // 長さ指定は最小と最大が同じ値
      length: Some(value),
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // Eメール形式のバリデーションを有効にするメソッド
  #[wasm_bindgen]
  pub fn email(&self) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: true,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // URL形式のバリデーションを有効にするメソッド
  #[wasm_bindgen]
  pub fn url(&self) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: true,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 正規表現のバリデーションを設定するメソッド
  #[wasm_bindgen]
  pub fn regex(&self, pattern: RegExp) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: Some(pattern),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 含むべき文字列を設定するメソッド
  #[wasm_bindgen]
  pub fn includes(&self, text: &str, options: &JsValue) -> ZodString {
    let mut position: Option<usize> = None;
    
    // オプションからpositionを取得
    if !options.is_undefined() && !options.is_null() {
      if let Ok(position_value) = js_sys::Reflect::get(options, &JsValue::from_str("position")) {
        if !position_value.is_undefined() && !position_value.is_null() {
          position = Some(position_value.as_f64().unwrap() as usize);
        }
      }
    }
    
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: Some(text.to_string()),
      includes_position: position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 先頭一致する文字列を設定するメソッド
  #[wasm_bindgen(js_name = startsWith)]
  pub fn starts_with(&self, text: &str) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: Some(text.to_string()),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 末尾一致する文字列を設定するメソッド
  #[wasm_bindgen(js_name = endsWith)]
  pub fn ends_with(&self, text: &str) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: Some(text.to_string()),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 空文字列を拒否するメソッド
  // Option<String>を使用（Option<&str>はwasm-bindgenでサポートされていないため）
  #[wasm_bindgen(js_name = nonempty)]
  pub fn nonempty(&self, message: Option<String>) -> ZodString {
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: Some(1),
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: self.regex.clone(),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: message,
    }
  }

  // base64 バリデーションを有効にするメソッド
  #[wasm_bindgen]
  pub fn base64(&self) -> ZodString {
    let base64_regex = RegExp::new(r"^(?:[A-Za-z0-9+\/]{4})*(?:[A-Za-z0-9+\/]{2}==|[A-Za-z0-9+\/]{3}=)?$", "");
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: Some(base64_regex),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // base64url バリデーションを有効にするメソッド
  #[wasm_bindgen]
  pub fn base64url(&self) -> ZodString {
    // https://github.com/colinhacks/zod/blob/850871defc2c98928f1c7e8e05e93d4a84ed3c5f/src/types.ts#L687
    let base64url_regex = RegExp::new(r"^([0-9a-zA-Z-_]{4})*(([0-9a-zA-Z-_]{2}(==)?)|([0-9a-zA-Z-_]{3}(=)?))?$", "");
    ZodString {
      base: ZodTypeBase::new(&self.base.type_name),
      min: self.min,
      max: self.max,
      length: self.length,
      is_email: self.is_email,
      is_url: self.is_url,
      regex: Some(base64url_regex),
      includes: self.includes.clone(),
      includes_position: self.includes_position,
      starts_with: self.starts_with.clone(),
      ends_with: self.ends_with.clone(),
      nonempty_message: self.nonempty_message.clone(),
    }
  }

  // 内部実装用のパースメソッド - トレイト実装のためのものではない
  fn _parse_internal(&self, value: &JsValue) -> JsValue {
    // 基本的な型チェック
    let base_result = <Self as ZodType>::_create_parse_result(self, value);
    let status = js_sys::Reflect::get(&base_result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "error" {
      return base_result;
    }
    
    // 値が文字列であることが確認できたら、制約をチェック
    if let Some(str_val) = value.as_string() {
      // 最小文字数のチェック
      if let Some(min_value) = self.min {
        if str_val.len() < min_value {
          // nonemptyのカスタムメッセージがあり、min_valueが1の場合はそれを使用
          let err_msg = if min_value == 1 && self.nonempty_message.is_some() {
            self.nonempty_message.as_ref().unwrap().clone()
          } else {
            format!("String must contain at least {} character(s)", min_value)
          };
          return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
        }
      }
      
      // 最大文字数のチェック
      if let Some(max_value) = self.max {
        if str_val.len() > max_value {
          let err_msg = format!("String must contain at most {} character(s)", max_value);
          return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
        }
      }
      
      // 正確な文字数のチェック
      if let Some(length_value) = self.length {
        if str_val.len() != length_value {
          let err_msg = format!("String must contain exactly {} character(s)", length_value);
          return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
        }
      }
      
      // Eメール形式のチェック
      if self.is_email {
        // 基本的なEメール形式のチェック
        // Zodの正規表現を利用 (https://github.com/colinhacks/zod/blob/850871defc2c98928f1c7e8e05e93d4a84ed3c5f/src/types.ts#L660)
        let email_regex = RegExp::new(r"^(?!\.)(?!.*\.\.)([A-Z0-9_'+\-\.]*)[A-Z0-9_+-]@([A-Z0-9][A-Z0-9\-]*\.)+[A-Z]{2,}$", "i");
        if !email_regex.test(&str_val) {
          return super::types::create_result_object("error", &JsValue::from_str("Invalid email"));
        }
      }
      
      // URL形式のチェック
      if self.is_url {
        // 基本的なURL形式のチェック
        // より柔軟なURLパターンに対応するよう、RFC3986に準拠した正規表現を使用
        let url_regex = RegExp::new(r"^[a-z]([a-z]|[0-9]|[+\-.])*:(\/\/((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|:)*@)?(\[((([0-9a-f]{1,4}:){6}([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|::([0-9a-f]{1,4}:){5}([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|([0-9a-f]{1,4})?::([0-9a-f]{1,4}:){4}([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|(([0-9a-f]{1,4}:){0,1}[0-9a-f]{1,4})?::([0-9a-f]{1,4}:){3}([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|(([0-9a-f]{1,4}:){0,2}[0-9a-f]{1,4})?::([0-9a-f]{1,4}:){2}([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|(([0-9a-f]{1,4}:){0,3}[0-9a-f]{1,4})?::[0-9a-f]{1,4}:([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|(([0-9a-f]{1,4}:){0,4}[0-9a-f]{1,4})?::([0-9a-f]{1,4}:[0-9a-f]{1,4}|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3})|(([0-9a-f]{1,4}:){0,5}[0-9a-f]{1,4})?::[0-9a-f]{1,4}|(([0-9a-f]{1,4}:){0,6}[0-9a-f]{1,4})?::)|v[0-9a-f]+\.(([a-z]|[0-9]|[-._~])|[!$&'()*+,;=]|:)+)]|([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\.([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])){3}|(([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=])*)(:\d*)?(\/((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@]))*)*|\/(((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@]))+(\/((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@]))*)*)?|((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@]))+(\/((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@]))*)*|)(\?((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@])|[\/?])*)?(#((([a-z]|[0-9]|[-._~])|%[0-9a-f][0-9a-f]|[!$&'()*+,;=]|[:@])|[\/?])*)?$", "i");
        if !url_regex.test(&str_val) {
          return super::types::create_result_object("error", &JsValue::from_str("Invalid url"));
        }
      }
      
      // 正規表現のチェック
      if let Some(regex_pattern) = &self.regex {
        if !regex_pattern.test(&str_val) {
          return super::types::create_result_object("error", &JsValue::from_str("Invalid string pattern"));
        }
      }
      
      // includesのチェック
      if let Some(includes_text) = &self.includes {
        // 位置指定がある場合
        if let Some(exact_position) = self.includes_position {
          // 指定位置から文字列が始まるかチェック
          if exact_position >= str_val.len() {
            // 開始位置が文字列長より大きい場合は見つからない
            let err_msg = format!("String must include \"{}\" at position {}", includes_text, exact_position);
            return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
          }
          
          // 指定位置から始まる部分文字列が指定の文字列で始まるかチェック
          if str_val.len() < exact_position + includes_text.len() {
            // 残りの文字数が足りない場合
            let err_msg = format!("String must include \"{}\" at position {}", includes_text, exact_position);
            return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
          }
          
          // exact_position位置から始まる部分が指定文字列と一致するかチェック
          let expected_substring = &str_val[exact_position..(exact_position + includes_text.len())];
          if expected_substring != includes_text {
            let err_msg = format!("String must include \"{}\" at position {}", includes_text, exact_position);
            return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
          }
        } else {
          // 位置指定がない場合は単純に含まれているかチェック
          if !str_val.contains(includes_text.as_str()) {
            let err_msg = format!("String must include \"{}\"", includes_text);
            return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
          }
        }
      }
      
      // startsWithのチェック
      if let Some(starts_with_text) = &self.starts_with {
        if !str_val.starts_with(starts_with_text) {
          let err_msg = format!("String must start with \"{}\"", starts_with_text);
          return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
        }
      }
      
      // endsWithのチェック
      if let Some(ends_with_text) = &self.ends_with {
        if !str_val.ends_with(ends_with_text) {
          let err_msg = format!("String must end with \"{}\"", ends_with_text);
          return super::types::create_result_object("error", &JsValue::from_str(&err_msg));
        }
      }
    }
    
    // すべての検証をパスしたら成功
    super::types::create_result_object("ok", value)
  }
  
  // ZodTypeトレイトの共通実装を使用するため、個別の実装は削除
}

// ZodString型にZodTypeトレイトを実装
impl ZodType for ZodString {
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
impl_js_methods!(ZodString);
