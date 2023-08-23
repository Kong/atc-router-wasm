use crate::{ast, schema::Schema};
use atc_router::context::Context as AtcContext;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue, UnwrapThrowExt};

#[wasm_bindgen]
pub struct Context {
    #[wasm_bindgen(skip)]
    pub c: *mut AtcContext<'static>,
}

#[wasm_bindgen]
impl Context {
    #[wasm_bindgen(constructor)]
    pub unsafe fn new(schema: &Schema) -> Context {
        Context {
            c: Box::into_raw(Box::new(AtcContext::new(&*schema.s))),
        }
    }

    #[wasm_bindgen(js_name = addValue)]
    pub unsafe fn add_value(&mut self, field: &str, value: ast::ExportedValue) {
        (*self.c).add_value(
            field,
            serde_wasm_bindgen::from_value(value.into()).unwrap_throw(),
        )
    }

    #[wasm_bindgen(js_name = valueOf)]
    pub unsafe fn value_of(&self, field: &str) -> ast::ExportedOptionValues {
        ast::ExportedOptionValues::from(
            (*self.c)
                .value_of(field)
                .map_or(JsValue::undefined(), |values| {
                    serde_wasm_bindgen::to_value(values).unwrap_throw()
                }),
        )
    }
}
