// src/lib.rs
use wasm_bindgen::prelude::*;
use uuid::Uuid;
use crate::message::{MessageHeader, PacketProtocol, StatusFlags};

// --- シリアライズ ---
#[wasm_bindgen]
pub fn wasm_serialize_packet(
    project_id_str: &str,
    device_id_str: &str, // <- 追加 (文字列として受け取る)
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

    // Project ID のパース
    let project_uuid = Uuid::parse_str(project_id_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid Project UUID: {}", e)))?;

    // Device ID のパース (追加)
    let device_uuid = Uuid::parse_str(device_id_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid Device UUID: {}", e)))?;

    let header = MessageHeader::new(
        project_uuid.into_bytes(),
        device_uuid.into_bytes(), // [u8; 16] に変換して渡す
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

// --- デシリアライズ ---
#[wasm_bindgen(getter_with_clone)]
pub struct WasmDecodedPacket {
    pub project_id: String,
    pub device_id: String, // <- String に変更
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

#[wasm_bindgen]
pub fn wasm_deserialize_packet(bytes: &[u8]) -> Result<WasmDecodedPacket, JsValue> {
    let (header, body) = PacketProtocol::deserialize(bytes)
        .map_err(|e| JsValue::from_str(e))?;

    // 16バイトの配列からUUID文字列を復元
    let project_uuid_str = Uuid::from_bytes(header.project_id).to_string();
    let device_uuid_str = Uuid::from_bytes(header.device_id).to_string(); // 追加

    Ok(WasmDecodedPacket {
        project_id: project_uuid_str,
        device_id: device_uuid_str, // 文字列をセット
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
