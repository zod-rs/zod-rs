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

// JavaScriptからの直接インスタンス化を防ぐため、
// privateコンストラクタをTypeScript定義と一致させる
impl ZodTypeBase {
  // このメソッドはJavaScriptからアクセスできない
  pub fn new(type_name: &str) -> Self {
    ZodTypeBase {
      type_name: type_name.to_string(),
    }
  }
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

  // 値をパースする内部メソッド（各実装で定義する必要あり）
  fn _parse(&self, value: &JsValue) -> JsValue;

  // 共通実装の parse メソッド - 成功時は値を返し、失敗時は例外をスロー
  fn parse(&self, value: &JsValue) -> JsValue {
    let result = self._parse(value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    if status.as_string().unwrap() == "ok" {
      // 成功した場合、値をそのまま返す
      js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap()
    } else {
      // エラーの場合は例外をスロー
      let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      let error_msg = if error_value.is_string() {
        error_value.as_string().unwrap()
      } else {
        format!("Expected {}, received {}", self.r#type(), self._get_type(value))
      };
      
      wasm_bindgen::throw_str(&error_msg);
    }
  }

  // 共通実装の safe_parse メソッド - JavaScriptの期待する結果形式を返す
  fn safe_parse(&self, value: &JsValue) -> JsValue {
    let result = self._parse(value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    
    let obj = js_sys::Object::new();
    
    if status.as_string().unwrap() == "ok" {
      // 成功した場合
      let parsed_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      js_sys::Reflect::set(&obj, &JsValue::from_str("success"), &JsValue::from_bool(true)).unwrap();
      js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &parsed_value).unwrap();
    } else {
      // エラーの場合
      let error_value = js_sys::Reflect::get(&result, &JsValue::from_str("value")).unwrap();
      let error_msg = if error_value.is_string() {
        error_value.as_string().unwrap()
      } else {
        format!("Expected {}, received {}", self.r#type(), self._get_type(value))
      };
      
      // エラーオブジェクトを作成
      let error_obj = js_sys::Object::new();
      let issues_array = js_sys::Array::new();
      let issue_obj = js_sys::Object::new();
      
      js_sys::Reflect::set(&issue_obj, &JsValue::from_str("message"), &JsValue::from_str(&error_msg)).unwrap();
      issues_array.push(&issue_obj);
      
      js_sys::Reflect::set(&error_obj, &JsValue::from_str("issues"), &issues_array).unwrap();
      
      js_sys::Reflect::set(&obj, &JsValue::from_str("success"), &JsValue::from_bool(false)).unwrap();
      js_sys::Reflect::set(&obj, &JsValue::from_str("error"), &error_obj).unwrap();
    }
    
    obj.into()
  }
}

// ZodTypeBaseにZodTypeトレイトを実装
impl ZodType for ZodTypeBase {
  fn r#type(&self) -> &str {
    &self.type_name
  }
  
  // 基本実装では単純な型チェックのみを行う
  fn _parse(&self, value: &JsValue) -> JsValue {
    self._create_parse_result(value)
  }
}

// 共通のJavaScriptインターフェース実装用マクロ
#[macro_export]
macro_rules! impl_js_methods {
  ($type:ty) => {
    #[wasm_bindgen]
    impl $type {
      // JavaScript用のparse実装
      #[wasm_bindgen]
      pub fn parse(&self, value: JsValue) -> JsValue {
        <Self as ZodType>::parse(self, &value)
      }

      // JavaScript用のsafe_parse実装
      #[wasm_bindgen(js_name = "safeParse")]
      pub fn safe_parse(&self, value: JsValue) -> JsValue {
        <Self as ZodType>::safe_parse(self, &value)
      }
    }
  };
}
