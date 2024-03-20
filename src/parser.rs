use atc_router::{ast::Expression, parser::parse, semantics::Validate};
use pest::error::{
    ErrorVariant as PestErrorVariant, InputLocation as PestInputLocation,
    LineColLocation as PestLineColLocation,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::schema::Schema;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export interface ParseError {
  variant: { parsingError: string } | { customError: string }
  location: { pos: number } | { span: [number, number] }
  lineCol: { pos: [number, number] } | { span: [[number, number], [number, number]] }
}

export interface ParseResultOk {
  status: 'ok'
  expression: any
}

export interface ParseResultParseError {
  status: 'parseError'
  parseError: ParseError
}

export interface ParseResultValidationError {
  status: 'validationError'
  expression: any
  validationError: string
}

export type ParseResult = ParseResultOk | ParseResultParseError | ParseResultValidationError
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = ParseResult)]
    pub type JsParseResult;
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
    pub variant: ErrorVariant,
    #[serde(with = "InputLocation")]
    pub location: PestInputLocation,
    #[serde(with = "LineColLocation", rename(serialize = "lineCol"))]
    pub line_col: PestLineColLocation,
}

#[derive(Serialize, Deserialize)]
pub struct ParseResultOk {
    pub status: &'static str,
    pub expression: Expression,
}

#[derive(Serialize, Deserialize)]
pub struct ParseResultParseError {
    pub status: &'static str,
    #[serde(with = "ParseError", rename(serialize = "parseError"))]
    pub parse_error: ParseError,
}

#[derive(Serialize, Deserialize)]
pub struct ParseResultValidationError {
    pub status: &'static str,
    pub expression: Expression,
    #[serde(rename(serialize = "validationError"))]
    pub validation_error: String,
}

#[wasm_bindgen]
pub struct Parser;

#[wasm_bindgen]
impl Parser {
    #[wasm_bindgen]
    pub unsafe fn parse(expression: &str, schema: &Schema) -> JsParseResult {
        match parse(expression) {
            Ok(expression) => match expression.validate(&*schema.s) {
                Ok(_) => serde_wasm_bindgen::to_value(&ParseResultOk {
                    status: "ok".into(),
                    expression,
                })
                .unwrap_throw()
                .into(),
                Err(err) => serde_wasm_bindgen::to_value(&ParseResultValidationError {
                    status: "validationError".into(),
                    expression,
                    validation_error: err.into(),
                })
                .unwrap_throw()
                .into(),
            },
            Err(err) => serde_wasm_bindgen::to_value(&ParseResultParseError {
                status: "parseError".into(),
                parse_error: ParseError {
                    variant: match err.variant {
                        PestErrorVariant::ParsingError {
                            positives: _,
                            negatives: _,
                        } => ErrorVariant::ParsingError(format!("{}", err.variant)),
                        PestErrorVariant::CustomError { message } => {
                            ErrorVariant::CustomError(message)
                        }
                    },
                    location: err.location,
                    line_col: err.line_col,
                },
            })
            .unwrap_throw()
            .into(),
        }
    }
}
