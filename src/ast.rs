use std::convert::From;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export type AstType = "String" | "IpCidr" | "IpAddr" | "Int" | "Regex";
export type AstValue = { String: string } | { IpCidr: string } | { IpAddr: string } | { Int: number } | { Regex: string };
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = AstType)]
    pub type JsAstType;

    #[wasm_bindgen(typescript_type = "AstType | undefined")]
    pub type JsOptionalAstType;

    #[wasm_bindgen(typescript_type = AstValue)]
    pub type JsAstValue;

    #[wasm_bindgen(typescript_type = "AstValue[] | undefined")]
    pub type JsOptionalAstValues;
}
