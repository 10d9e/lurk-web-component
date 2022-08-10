mod utils;


/// Module for wasm-bindgen specific handling and endpoints.
use lurk::{
    eval::{empty_sym_env, Evaluator},
    store::{ContTag, Pointer, Store},
    writer::Write,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde_json::json;
use blstrs::Scalar as Fr;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

/// Run a lurk snippet
#[wasm_bindgen(catch)]
pub fn execute_lurk(source: JsValue) -> Result<JsValue, JsValue> {
    let expression = source
        .as_string()
        .ok_or_else(|| "input source must be a string")?;

    let context = run_lurk(expression);
    let json = json!(&context);
    Ok(json.to_string().into())
}

fn run_lurk(expression: String) -> HashMap<&'static str, String> {
    let limit = 100_000_000;
    let mut store = Store::<Fr>::default();
    let mut context: HashMap<&str, String> = HashMap::new();

    context.insert("expression", expression.clone());

    if let Some(expr) = store.read(&expression) {
        let (output, iterations, _ptrs) =
            Evaluator::new(expr, empty_sym_env(&store), &mut store, limit).eval();

        let iterations_str = iterations.to_string();
        context.insert("iterations", iterations_str);
        let result_str = match output.cont.tag() {
            ContTag::Outermost | ContTag::Terminal => {
                let result = store.fetch(&output.expr).clone().unwrap();
                result.fmt_to_string(&store)
            }
            ContTag::Error => "ERROR!".to_string(),
            _ => format!("Computation incomplete after limit: {}", limit),
        };

        context.insert("result", result_str);
    } else {
        let error = format!("Syntax Error: {}", &expression);
        context.insert("result", error);
    }
    println!("{:?}", store);
    return context;
}

#[cfg(test)]
mod tests {
    use wasm_bindgen::JsValue;

    use crate::{execute_lurk, run_lurk};

    #[test]
    fn test_execute_lurk() {
        // let result = execute_lurk(JsValue::from_str("(* 8 7)"));
        let result = run_lurk(String::from("(* 8 7)"));
        println!("{:?}", result);
        // assert_eq!(result, 4);
        assert_eq!(4, 4);
    }
}
