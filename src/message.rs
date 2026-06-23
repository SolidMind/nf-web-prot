use std::fmt::Debug;
use std::mem::size_of;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes};

use crate::VERSION;

#[repr(u8)]
#[derive(
    Debug, Default, PartialEq, Immutable, IntoBytes, TryFromBytes, KnownLayout, Clone, Copy,
)]
pub enum BodyType {
    #[default]
    Empty = 0,
    Test = 1,
    Sensor = 2,
    Media = 3,
    Move = 4,
    Silence = 5,
    Connect = 6,
    Disconnect = 7,
    Dynamic = u8::MAX,
}

#[derive(Debug, TryFromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct MessageHeader {
    pub project_id: [u8; 16], // 16 bytes (Offset 0)
    pub device_id: [u8; 16],  // 16 bytes (Offset 16)
    pub r#type: BodyType,     // 2 bytes (Offset 32)
    pub version: u8,          // 1 bytes (Offset 33)
    pub _padding: [u8; 6],    // 6 bytes (Offset 34)
    pub time: i64,            // 8 bytes (Offset 40)
}

//
// 実際のボディ構造体の定義
//


#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct EmptyBody {}

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct MessageDataBody {
    pub mask_index: [u8; 8],
    pub mask_white_ratio: [f32; 8],
    pub interval_ms: u32,
}

/// 受信したパケットをいったん保持するための共通構造体。
/// ジェネリクスを持たないため、配列(Vec)にまとめたり、キューに入れたりしやすくなる。
#[derive(Debug, Clone)]
pub struct RawMessage {
    pub header: MessageHeader,
    pub raw_body: Vec<u8>, // ボディは型にはめず、生のバイト列として保持
}

impl RawMessage {
    /// バイト列から RawMessage を生成する（デシリアライズ / 受信時）
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let header_size = size_of::<MessageHeader>();
        if bytes.len() < header_size {
            return Err("Packet too small to contain header");
        }

        // ヘッダーのみ先にパース
        let header = MessageHeader::try_read_from_bytes(&bytes[..header_size])
            .map_err(|_| "Failed to parse header")?;

        Ok(Self {
            header,
            raw_body: bytes[header_size..].to_vec(), // 残りをすべてボディとして保存
        })
    }

    /// RawMessage をバイト列に変換する（シリアライズ / 送信時）
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(size_of::<MessageHeader>() + self.raw_body.len());
        buffer.extend_from_slice(self.header.as_bytes());
        buffer.extend_from_slice(&self.raw_body);
        buffer
    }

    pub fn new(
        project_id: [u8; 16],
        device_id: [u8; 16],
        time: i64,
        body_type: BodyType,
    ) -> RawMessage {
        Self {
            header: MessageHeader {
                project_id,
                device_id,
                r#type: body_type,
                version: VERSION,
                _padding: [0; 6],
                time,
            },
            raw_body: Vec::new(),
        }
    }

    pub fn new_msg(
        project_id: [u8; 16],
        device_id: [u8; 16],
        time: i64,
        body_type: BodyType,
        body: MessageDataBody,
    ) -> RawMessage {
        Self {
            header: MessageHeader {
                project_id,
                device_id,
                r#type: body_type,
                version: VERSION,
                _padding: [0; 6],
                time,
            },
            raw_body: body.as_bytes().to_vec(),
        }
    }
    pub fn extract_message_body(&self) -> Option<MessageDataBody> {
        if self.header.r#type != BodyType::Move || self.header.r#type != BodyType::Silence {
            return None;
        }
        // FromBytesが実装されているので、サイズが合えば安全に読み取れる
        MessageDataBody::read_from_bytes(self.raw_body.as_slice()).ok()
    }
}
