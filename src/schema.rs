use crate::ast;
use atc_router::schema::Schema as AtcSchema;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Schema {
    #[wasm_bindgen(skip)]
    pub s: *mut AtcSchema,
}

#[wasm_bindgen]
impl Schema {
    #[wasm_bindgen(constructor)]
    pub unsafe fn new() -> Schema {
        Schema {
            s: Box::into_raw(Box::new(AtcSchema::default())),
        }
    }

    #[wasm_bindgen(js_name = addField)]
    pub unsafe fn add_field(&mut self, field: &str, typ: ast::ExportedType) {
        (*self.s).add_field(
            field,
            serde_wasm_bindgen::from_value(typ.into()).unwrap_throw(),
        );
    }

    #[wasm_bindgen(js_name = typeOf)]
    pub unsafe fn type_of(&self, field: &str) -> ast::ExportedOptionType {
        (*self.s)
            .type_of(field)
            .map_or(JsValue::undefined(), |t| {
                serde_wasm_bindgen::to_value(t).unwrap_throw()
            })
            .into()
    }
}
