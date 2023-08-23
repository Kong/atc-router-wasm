use std::convert::From;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export type AstType = "String" | "IpCidr" | "IpAddr" | "Int" | "Regex" | undefined;
export type AstValue = { String: string } | { IpCidr: string } | { IpAddr: string } | { Regex: string } | { Int: number };
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = AstType)]
    pub type ExportedType;

    #[wasm_bindgen(typescript_type = "AstType | undefined")]
    pub type ExportedOptionType;

    #[wasm_bindgen(typescript_type = AstValue)]
    pub type ExportedValue;

    #[wasm_bindgen(typescript_type = "AstValue[] | undefined")]
    pub type ExportedOptionValues;
}
