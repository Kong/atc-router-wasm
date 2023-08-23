use atc_router::{ast::Expression, parser::parse};
use pest::error::{
    ErrorVariant as PestErrorVariant, InputLocation as PestInputLocation,
    LineColLocation as PestLineColLocation,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const _INJECTED_TYPESCRIPT: &'static str = r#"
export type ParserParseResult = {
  ok: true;
  expressions: any;
};

export type ParserParseError = {
  ok: false;
  variant:
    | {
        parseError: {
          message: string;
        };
      }
    | {
        customError: {
          message: string;
        };
      };
  location: { Pos: number } | { Span: [number, number] };
  lineCol:
    | { Pos: [number, number] }
    | { Span: [[number, number], [number, number]] };
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParserParseResult | ParserParseError")]
    pub type ExportedParseResult;
}

#[derive(Serialize, Deserialize)]
pub enum ErrorVariant {
    #[serde(rename(serialize = "parsingError"))]
    ParsingError { message: String },
    #[serde(rename(serialize = "customError"))]
    CustomError { message: String },
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PestInputLocation")]
enum InputLocation {
    Pos(usize),
    Span((usize, usize)),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "PestLineColLocation")]
enum LineColLocation {
    Pos((usize, usize)),
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
                    } => ErrorVariant::ParsingError {
                        message: format!("{}", err.variant),
                    },
                    PestErrorVariant::CustomError { message } => {
                        ErrorVariant::CustomError { message }
                    }
                },
                location: err.location,
                line_col: err.line_col,
            }),
        }
        .unwrap_throw()
        .into()
    }
}
