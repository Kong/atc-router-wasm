use crate::{context::Context, error::ExportedErrorMessage, schema::Schema};
use atc_router::router::Router as AtcRouter;
use std::str::FromStr;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Router {
    #[wasm_bindgen(skip)]
    pub r: *mut AtcRouter<'static>,
}

#[wasm_bindgen]
impl Router {
    #[wasm_bindgen(constructor)]
    pub unsafe fn new(schema: &Schema) -> Router {
        Router {
            r: Box::into_raw(Box::new(AtcRouter::new(&*schema.s))),
        }
    }

    #[wasm_bindgen(js_name = addMatcher)]
    pub unsafe fn add_matcher(
        &mut self,
        priority: usize,
        uuid: &str,
        atc: &str,
    ) -> ExportedErrorMessage {
        match (*self.r).add_matcher(priority, Uuid::from_str(uuid).unwrap_throw(), atc) {
            Ok(_) => JsValue::undefined(),
            Err(err) => err.into(),
        }
        .into()
    }

    #[wasm_bindgen(js_name = removeMatcher)]
    pub unsafe fn remove_matcher(&mut self, priority: usize, uuid: &str) -> bool {
        (*self.r).remove_matcher(priority, Uuid::from_str(uuid).unwrap_throw())
    }

    #[wasm_bindgen]
    pub unsafe fn execute(&self, context: &mut Context) -> bool {
        (*self.r).execute(&mut *context.c)
    }
}
