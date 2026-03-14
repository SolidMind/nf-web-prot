use wasm_bindgen::prelude::*;
use crate::message::{MessageHeader, PacketProtocol, StatusFlags};

#[wasm_bindgen]
pub fn wasm_serialize_packet(
    project_id_hi: u64,
    project_id_lo: u64,
    device_id_hi: u64,
    device_id_ho:u64,
    time: i64,
    interval_ms: u32,
    mask_white_ratio: f32,
    codec_array: &[u8],
    body_size: u32,
    state_flag: u16,
    mask_index: u8,
    body: &[u8],
) -> Result<Vec<u8>, JsValue> {
    if codec_array.len() != 4 {
        return Err(JsValue::from_str("Codec must be exactly 4 bytes"));
    }

    let mut codec = [0u8; 4];
    codec.copy_from_slice(&codec_array[0..4]);

    // JSから渡された上位64bitと下位64bitを結合してu128にする
    let project_id = ((project_id_hi as u128) << 64) | (project_id_lo as u128);
    let device_id = ((device_id_hi as u128) << 64) | (device_id_ho as u128);

    let header = MessageHeader::new(
        project_id,
        device_id,
        time,
        StatusFlags(state_flag),
        mask_index,
        mask_white_ratio,
        interval_ms,
        codec,
        body_size,
    );

    Ok(PacketProtocol::serialize(&header, body))
}

// JS/TS側で受け取るための構造体
#[wasm_bindgen(getter_with_clone)]
pub struct WasmDecodedPacket {
    pub project_id_hi: u64,
    pub project_id_lo: u64,
    pub device_id_hi: u64,
    pub device_id_lo: u64,
    pub time: i64,
    pub interval_ms: u32,
    pub mask_white_ratio: f32,
    pub state_flag: u16,
    pub mask_index: u8,
    pub body_size: u32,
    codec_internal: [u8; 4],
    body_internal: Vec<u8>,
}

#[wasm_bindgen]
impl WasmDecodedPacket {
    #[wasm_bindgen(getter)]
    pub fn codec(&self) -> Vec<u8> {
        self.codec_internal.to_vec()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Vec<u8> {
        self.body_internal.clone()
    }
}

// デシリアライズ関数
#[wasm_bindgen]
pub fn wasm_deserialize_packet(bytes: &[u8]) -> Result<WasmDecodedPacket, JsValue> {
    let (header, body) = PacketProtocol::deserialize(bytes)
        .map_err(|e| JsValue::from_str(e))?;

    let project_id_hi = (header.project_id >> 64) as u64;
    let project_id_lo = header.project_id as u64;

    let device_id_hi = (header.device_id >> 64) as u64;
    let device_id_lo = header.device_id as u64;

    Ok(WasmDecodedPacket {
        project_id_hi,
        project_id_lo,
        device_id_hi,
        device_id_lo,
        time: header.time,
        interval_ms: header.interval_ms,
        mask_white_ratio: header.mask_white_ratio,
        state_flag: header.state.0,
        mask_index: header.mask_index,
        body_size: header.body_size,
        codec_internal: header.codec,
        body_internal: body.to_vec(),
    })
}
