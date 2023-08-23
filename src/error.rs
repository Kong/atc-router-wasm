use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export type ErrorMessage = string | undefined;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = ErrorMessage)]
    pub type ExportedErrorMessage;
}
