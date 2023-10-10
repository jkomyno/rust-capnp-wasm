mod capnp;
mod event;

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::event::Event;

#[wasm_bindgen(js_name = "createEvent")]
pub fn create_event(event: Event) -> Result<Vec<u8>, JsValue> {
    capnp::create_event(event).map_err(|e| e.extra.into())
}

#[wasm_bindgen(js_name = "modifyEvent")]
pub fn modify_event(event: &mut [u8]) -> Result<(), JsValue> {
    capnp::modify_event(event).map_err(|e| e.extra.into())
}

#[wasm_bindgen(js_name = "readEvent")]
pub fn read_event(event: &[u8]) -> Result<Event, JsValue> {
    capnp::read_event(event).map_err(|e| e.extra.into())
}
