mod utils;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

static INTERPRETER: Lazy<Mutex<silang::silang::Interpreter>> =
    Lazy::new(|| Mutex::new(silang::silang::Interpreter::new()));

#[wasm_bindgen]
#[derive(Clone)]
pub enum SilangStatus {
    ParseError = 0,
    Normal = 1,
    Error = 2,
}
#[wasm_bindgen]
pub struct SilangResult {
    status: SilangStatus,
    output: String,
}

#[wasm_bindgen]
impl SilangResult {
    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        self.output.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn status(&self) -> SilangStatus {
        self.status.clone()
    }
}

#[wasm_bindgen]
pub fn welcome_message() -> String {
    utils::set_panic_hook();
    format!(
        "SILang Interpreter Ver:{}",
        INTERPRETER.lock().unwrap().version
    )
}

#[wasm_bindgen]
pub fn exec(code: &str) -> SilangResult {
    match silang::preprocessor::preprocess(code) {
        Ok(preprocessed_code) => {
            match silang::parser::statement_all_consuming(&preprocessed_code) {
                Ok(statement) => {
                    if let Err(e) = INTERPRETER.lock().unwrap().exec(&statement.1) {
                        return SilangResult {
                            status: SilangStatus::Error,
                            output: format!("{}\n", e),
                        };
                    }
                    let output = INTERPRETER.lock().unwrap().buffer_flush();
                    return SilangResult {
                        status: SilangStatus::Normal,
                        output: output,
                    };
                }
                Err(_) => {
                    return SilangResult {
                        status: SilangStatus::ParseError,
                        output: "".to_owned(),
                    }
                }
            }
        }
        Err(_) => {
            return SilangResult {
                status: SilangStatus::ParseError,
                output: "".to_owned(),
            }
        }
    }
}
