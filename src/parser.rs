use atc_router::{ast::Expression, parser::parse};
use pest::error::{
    ErrorVariant as PestErrorVariant, InputLocation as PestInputLocation,
    LineColLocation as PestLineColLocation,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export interface ParserParseOk {
  ok: true;
  expressions: any;
}

export interface ParserParseError {
  ok: false;
  variant: { parsingError: string } | { customError: string };
  location: { pos: number } | { span: [number, number] };
  lineCol:
    | { pos: [number, number] }
    | { span: [[number, number], [number, number]] };
}

export type ParserParseResult = ParserParseOk | ParserParseError;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParserParseResult")]
    pub type ExportedParseResult;
}

#[derive(Serialize, Deserialize)]
pub enum ErrorVariant {
    #[serde(rename(serialize = "parsingError"))]
    ParsingError(String),
    #[serde(rename(serialize = "customError"))]
    CustomError(String),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PestInputLocation")]
enum InputLocation {
    #[serde(rename(serialize = "pos"))]
    Pos(usize),
    #[serde(rename(serialize = "span"))]
    Span((usize, usize)),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PestLineColLocation")]
enum LineColLocation {
    #[serde(rename(serialize = "pos"))]
    Pos((usize, usize)),
    #[serde(rename(serialize = "span"))]
    Span((usize, usize), (usize, usize)),
}

#[derive(Serialize, Deserialize)]
pub struct ParseError {
    pub ok: bool,
    pub variant: ErrorVariant,
    #[serde(with = "InputLocation")]
    pub location: PestInputLocation,
    #[serde(with = "LineColLocation", rename(serialize = "lineCol"))]
    pub line_col: PestLineColLocation,
}

// Synthetic Result<T>
#[derive(Serialize, Deserialize)]
pub struct ParseResult {
    pub ok: bool,
    pub expressions: Expression,
}

#[wasm_bindgen]
pub struct Parser;

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen]
    pub unsafe fn parse(expressions: &str) -> ExportedParseResult {
        match parse(expressions) {
            Ok(expressions) => serde_wasm_bindgen::to_value(&ParseResult {
                ok: true,
                expressions,
            }),
            Err(err) => serde_wasm_bindgen::to_value(&ParseError {
                ok: false,
                variant: match err.variant {
                    PestErrorVariant::ParsingError {
                        positives: _,
                        negatives: _,
                    } => ErrorVariant::ParsingError(format!("{}", err.variant)),
                    PestErrorVariant::CustomError { message } => ErrorVariant::CustomError(message),
                },
                location: err.location,
                line_col: err.line_col,
            }),
        }
        .unwrap_throw()
        .into()
    }
}
