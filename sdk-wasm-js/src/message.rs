use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub(crate) inner: trezoa_message::Message,
}

crate::conversion::impl_inner_conversion!(Message, trezoa_message::Message);
