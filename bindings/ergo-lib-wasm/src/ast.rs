//! Ergo constant values

use crate::utils::I64;
use ergo_lib::chain::Base16Str;
use ergo_lib::ergotree_ir::mir::constant::TryExtractFrom;
use js_sys::Uint8Array;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

extern crate derive_more;
use derive_more::{From, Into};

/// Ergo constant(evaluated) values
#[wasm_bindgen]
#[derive(PartialEq, Eq, Debug, Clone, From, Into)]
pub struct Constant(ergo_lib::ergotree_ir::mir::constant::Constant);

#[wasm_bindgen]
impl Constant {
    /// Decode from Base16-encoded ErgoTree serialized value
    pub fn decode_from_base16(base16_bytes_str: String) -> Result<Constant, JsValue> {
        let bytes = ergo_lib::chain::Base16DecodedBytes::try_from(base16_bytes_str.clone())
            .map_err(|_| {
                JsValue::from_str(&format!(
                    "failed to decode base16 from: {}",
                    base16_bytes_str.clone()
                ))
            })?;
        ergo_lib::ergotree_ir::mir::constant::Constant::try_from(bytes)
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(Constant)
    }

    /// Encode as Base16-encoded ErgoTree serialized value
    pub fn encode_to_base16(&self) -> String {
        self.0.base16_str()
    }

    /// Create from i32 value
    pub fn from_i32(v: i32) -> Constant {
        Constant(v.into())
    }

    /// Extract i32 value, returning error if wrong type
    pub fn to_i32(&self) -> Result<i32, JsValue> {
        i32::try_extract_from(self.0.clone()).map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }

    /// Create from i64
    pub fn from_i64(v: &I64) -> Constant {
        Constant(i64::from((*v).clone()).into())
    }

    /// Extract i64 value, returning error if wrong type
    pub fn to_i64(&self) -> Result<I64, JsValue> {
        i64::try_extract_from(self.0.clone())
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(I64::from)
    }

    /// Create from byte array
    pub fn from_byte_array(v: &[u8]) -> Constant {
        Constant(v.to_vec().into())
    }

    /// Extract byte array, returning error if wrong type
    pub fn to_byte_array(&self) -> Result<Uint8Array, JsValue> {
        Vec::<u8>::try_extract_from(self.0.clone())
            .map(|v| Uint8Array::from(v.as_slice()))
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }
}
