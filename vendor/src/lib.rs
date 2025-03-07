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
pub fn create_zod_number() -> zod::ZodNumber {
    zod::ZodNumber::new()
}

#[wasm_bindgen]
pub fn create_zod_string() -> zod::ZodString {
    zod::ZodString::new()
}

// JavaScriptのコールバック関数を作成するヘルパー
fn create_js_callback<F>(f: F) -> js_sys::Function 
where
    F: 'static + Fn() -> JsValue,
{
    let closure = wasm_bindgen::closure::Closure::wrap(
        Box::new(f) as Box<dyn Fn() -> JsValue>
    );
    let js_func = closure.as_ref().clone();
    closure.forget(); // メモリリークを防止するためJavaScriptに所有権を移譲
    js_func.into()
}

// 型情報を表す構造体
struct ZodTypeInfo {
    name: &'static str,
    factory: fn() -> JsValue,
}

// zodのzオブジェクトを作成する
#[wasm_bindgen]
pub fn create_zod() -> JsValue {
    let z = js_sys::Object::new();
    
    // Zodの型定義のリスト
    // 新しい型を追加するには、この配列に要素を追加するだけ
    let types = [
        ZodTypeInfo {
            name: "number",
            factory: || JsValue::from(create_zod_number()),
        },
        ZodTypeInfo {
            name: "string",
            factory: || JsValue::from(create_zod_string()),
        },
        // 例: 将来的な追加
        // ZodTypeInfo {
        //     name: "boolean",
        //     factory: || JsValue::from(create_zod_boolean()),
        // },
    ];
    
    // 各型をzオブジェクトに登録
    for type_info in types.iter() {
        let factory_fn = create_js_callback(type_info.factory);
        js_sys::Reflect::set(
            &z,
            &JsValue::from_str(type_info.name),
            &factory_fn
        ).unwrap();
    }
    
    z.into()
}
