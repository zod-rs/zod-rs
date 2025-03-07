mod utils;
mod zod;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

#[wasm_bindgen]
pub fn is_number(value: JsValue) -> bool {
    let result = zod::ZodNumber::new()._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    status.as_string().unwrap() == "ok"
}

#[wasm_bindgen]
pub fn is_string(value: JsValue) -> bool {
    let result = zod::ZodString::new()._parse(&value);
    let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
    status.as_string().unwrap() == "ok"
}

// 安全に文字列として取得するヘルパー
#[wasm_bindgen]
pub fn safe_parse_string(value: JsValue) -> Result<JsValue, JsValue> {
    match zod::ZodString::new()._parse(&value) {
        result => {
            let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
            if status.as_string().unwrap() == "ok" {
                Ok(value)
            } else {
                Err(JsValue::from_str("Invalid string"))
            }
        }
    }
}

// 安全に数値として取得するヘルパー
#[wasm_bindgen]
pub fn safe_parse_number(value: JsValue) -> Result<JsValue, JsValue> {
    match zod::ZodNumber::new()._parse(&value) {
        result => {
            let status = js_sys::Reflect::get(&result, &JsValue::from_str("status")).unwrap();
            if status.as_string().unwrap() == "ok" {
                Ok(value)
            } else {
                Err(JsValue::from_str("Invalid number"))
            }
        }
    }
}

#[wasm_bindgen]
pub fn create_zod_number() -> zod::ZodNumber {
    zod::ZodNumber::new()
}

#[wasm_bindgen]
pub fn create_zod_string() -> zod::ZodString {
    zod::ZodString::new()
}

// zodのzオブジェクトを作成する
#[wasm_bindgen]
pub fn create_zod() -> JsValue {
    let z = js_sys::Object::new();
    js_sys::Reflect::set(
        &z,
        &JsValue::from_str("number"),
        &JsValue::from(create_zod_number)
    ).unwrap();
    js_sys::Reflect::set(
        &z,
        &JsValue::from_str("string"),
        &JsValue::from(create_zod_string)
    ).unwrap();
    z.into()
}
