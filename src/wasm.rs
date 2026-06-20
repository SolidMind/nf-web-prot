use crate::message::{
    BodyType, MediaDataMeta, MessageDataBody, MessageHeader, RawMessage, SensorDataBody,
};
use serde::Serialize;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use zerocopy::{FromBytes, IntoBytes};

static VERSION: u8 = 2u8;

// --- シリアライズ ---
#[wasm_bindgen]
pub fn wasm_serialize_packet(
    project_id_str: &str,
    device_id_str: &str,
    time: i64,
    body_type: u8, // state_flagの代わりにBodyTypeの値(u8)を受け取る
    // Move (MessageDataBody) 用
    interval_ms: u32,
    mask_white_ratio_array: &[f32],
    mask_index_array: &[u8],
    // Media 用
    codec_array: &[u8],
    // 可変長ボディ (Mediaのデータなど)
    body: &[u8],
) -> Result<Vec<u8>, JsValue> {
    // 1. UUIDのパース
    let project_uuid = Uuid::parse_str(project_id_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid Project UUID: {}", e)))?;
    let device_uuid = Uuid::parse_str(device_id_str)
        .map_err(|e| JsValue::from_str(&format!("Invalid Device UUID: {}", e)))?;

    // 2. ヘッダーの共通部分を生成
    let mut header = MessageHeader {
        project_id: project_uuid.into_bytes(),
        device_id: device_uuid.into_bytes(),
        r#type: BodyType::Empty, // 後で上書きします
        version: VERSION,
        _padding: [0; 6],
        time,
    };

    // 3. BodyType に応じてボディを組み立てる
    let raw_body = match body_type {
        // BodyType::Sensor (2) の場合
        2 => {
            header.r#type = BodyType::Sensor;
            // Wasmの引数にセンサー値が追加されたと仮定するか、ダミーを入れる
            let sensor_body = SensorDataBody {
                temperature: 0.0,
                humidity: 0.0,
            };
            sensor_body.as_bytes().to_vec()
        }
        // BodyType::Media (3) の場合
        3 => {
            if codec_array.len() != 4 {
                return Err(JsValue::from_str("Codec must be exactly 4 bytes"));
            }
            header.r#type = BodyType::Media;

            let mut codec = [0u8; 4];
            codec.copy_from_slice(&codec_array[0..4]);

            let meta = MediaDataMeta {
                codec,
                data_length: body.len() as u32,
            };

            // メタデータとペイロードを結合
            let mut buffer = meta.as_bytes().to_vec();
            buffer.extend_from_slice(body);
            buffer
        }
        // BodyType::Move (4) の場合
        4 => {
            if mask_white_ratio_array.len() != 8 {
                return Err(JsValue::from_str(
                    "mask_white_ratio must be exactly 8 elements",
                ));
            }
            if mask_index_array.len() != 8 {
                return Err(JsValue::from_str("mask_index must be exactly 8 elements"));
            }
            header.r#type = BodyType::Move;

            let mut mask_white_ratio = [0f32; 8];
            mask_white_ratio.copy_from_slice(&mask_white_ratio_array[0..8]);

            let mut mask_index = [0u8; 8];
            mask_index.copy_from_slice(&mask_index_array[0..8]);

            let msg_body = MessageDataBody {
                mask_index,
                mask_white_ratio,
                interval_ms,
            };
            msg_body.as_bytes().to_vec()
        }
        _ => {
            header.r#type = BodyType::Empty;
            vec![] // 空のボディ
        }
    };

    let raw_message = RawMessage { header, raw_body };
    Ok(raw_message.serialize())
}

//
// デシリアライズ
//
#[wasm_bindgen(getter_with_clone)]
pub struct WasmDecodedPacket {
    pub project_id: String,
    pub device_id: String,
    pub time: i64,
    pub body_type: u8,
    // 取得されなかった場合は空配列を返すための内部データ
    interval_ms_internal: Option<u32>,
    mask_white_ratio_internal: Option<Vec<f32>>,
    mask_index_internal: Option<Vec<u8>>,
    codec_internal: Option<Vec<u8>>,
    body_internal: Vec<u8>,
}

#[wasm_bindgen]
impl WasmDecodedPacket {
    #[wasm_bindgen(getter)]
    pub fn interval_ms(&self) -> u32 {
        self.interval_ms_internal.unwrap_or(0)
    }
    #[wasm_bindgen(getter)]
    pub fn codec(&self) -> Vec<u8> {
        self.codec_internal.clone().unwrap_or_default()
    }
    #[wasm_bindgen(getter)]
    pub fn mask_white_ratio(&self) -> Vec<f32> {
        self.mask_white_ratio_internal.clone().unwrap_or_default()
    }
    #[wasm_bindgen(getter)]
    pub fn mask_index(&self) -> Vec<u8> {
        self.mask_index_internal.clone().unwrap_or_default()
    }
    #[wasm_bindgen(getter)]
    pub fn body(&self) -> Vec<u8> {
        self.body_internal.clone()
    }
}

#[wasm_bindgen]
pub fn wasm_deserialize_packet(bytes: &[u8]) -> Result<WasmDecodedPacket, JsValue> {
    let raw_msg = RawMessage::parse(bytes).map_err(JsValue::from_str)?;

    let mut packet = WasmDecodedPacket {
        project_id: Uuid::from_bytes(raw_msg.header.project_id).to_string(),
        device_id: Uuid::from_bytes(raw_msg.header.device_id).to_string(),
        time: raw_msg.header.time,
        body_type: raw_msg.header.r#type as u8,
        interval_ms_internal: None,
        mask_white_ratio_internal: None,
        mask_index_internal: None,
        codec_internal: None,
        body_internal: raw_msg.raw_body.clone(),
    };

    // BodyTypeごとに専用のデータを抽出
    match raw_msg.header.r#type {
        BodyType::Media => {
            if let Some((meta, payload)) = raw_msg.extract_media_data() {
                packet.codec_internal = Some(meta.codec.to_vec());
                packet.body_internal = payload.to_vec(); // メタデータを省いた純粋なデータ
            }
        }
        BodyType::Move => {
            // MessageDataBody としてのパース
            if let Ok(msg_data) = MessageDataBody::read_from_bytes(raw_msg.raw_body.as_slice()) {
                packet.mask_index_internal = Some(msg_data.mask_index.to_vec());
                packet.mask_white_ratio_internal = Some(msg_data.mask_white_ratio.to_vec());
                packet.interval_ms_internal = Some(msg_data.interval_ms);
            }
        }
        _ => {}
    }

    Ok(packet)
}

//
// デシリアライズ (JSON 変換用)
//

// JSに渡したいデータ構造を定義（使わない項目は生成しない設定）
#[derive(Serialize)]
pub struct DecodedResult {
    pub project_id: String,
    pub device_id: String,
    pub time: i64,
    pub body_type: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_white_ratio: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_index: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>, // 文字列として返す
    pub body: Vec<u8>, // Uint8Arrayになる
}

#[wasm_bindgen]
pub fn wasm_deserialize_to_json(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let raw_msg = RawMessage::parse(bytes).map_err(JsValue::from_str)?;

    let mut result = DecodedResult {
        project_id: Uuid::from_bytes(raw_msg.header.project_id).to_string(),
        device_id: Uuid::from_bytes(raw_msg.header.device_id).to_string(),
        time: raw_msg.header.time,
        body_type: raw_msg.header.r#type as u8,
        interval_ms: None,
        mask_white_ratio: None,
        mask_index: None,
        codec: None,
        body: raw_msg.raw_body.clone(),
    };

    match raw_msg.header.r#type {
        BodyType::Media => {
            if let Some((meta, payload)) = raw_msg.extract_media_data() {
                result.codec = Some(String::from_utf8_lossy(&meta.codec).to_string());
                result.body = payload.to_vec();
            }
        }
        BodyType::Move => {
            if let Ok(msg_data) = MessageDataBody::read_from_bytes(raw_msg.raw_body.as_slice()) {
                result.mask_index = Some(msg_data.mask_index.to_vec());
                result.mask_white_ratio = Some(msg_data.mask_white_ratio.to_vec());
                result.interval_ms = Some(msg_data.interval_ms);
            }
        }
        _ => {}
    }

    // serde-wasm-bindgen で Rust の構造体を直接 JS オブジェクトに変換
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
