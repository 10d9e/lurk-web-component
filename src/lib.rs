mod utils;

use wasm_bindgen::prelude::*;
use anyhow::Result;
// use lurk::repl::repl;
use lurk::eval::Evaluator;

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

#[wasm_bindgen]
pub fn evaluate(src: &src) {
    let mut s = Store::<Fr>::default();
    let limit = 1000000;
    let expr = src;

    let (
        IO {
            expr: _result_expr,
            env: _new_env,
            cont: _continuation,
        },
        iterations,
        _emitted,
    ) = Evaluator::new(expr, empty_sym_env(&s), &mut s, limit).eval();

    //repl(src)
}