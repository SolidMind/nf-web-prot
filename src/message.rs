use std::fmt::Debug;
use std::mem::size_of;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, TryFromBytes};

static VERSION: u8 = 2u8;

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

// ==========================================
// 実際のボディ構造体の定義
// ==========================================

// トレイトはマーカーとして残しておいても良いですが、
// ジェネリクスを使わないため必須ではありません。
pub trait MessageBody: FromBytes + IntoBytes + Immutable + KnownLayout + Debug + Clone {}

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct EmptyBody {}
impl MessageBody for EmptyBody {}

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct SensorDataBody {
    pub temperature: f32,
    pub humidity: f32,
}
impl MessageBody for SensorDataBody {}

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct MediaDataMeta {
    pub codec: [u8; 4],
    pub data_length: u32,
}
impl MessageBody for MediaDataMeta {}

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct MessageDataBody {
    pub mask_index: [u8; 8],
    pub mask_white_ratio: [f32; 8],
    pub interval_ms: u32,
}
impl MessageBody for MessageDataBody {}

/// 受信したパケットをいったん保持するための共通構造体。
/// ジェネリクスを持たないため、配列(Vec)にまとめたり、キューに入れたりしやすくなる。
#[derive(Debug, Clone)]
pub struct RawMessage {
    pub header: MessageHeader,
    pub raw_body: Vec<u8>, // ボディは型にはめず、生のバイト列として保持
}

impl RawMessage {
    /// 1. 送信用のコンストラクタ（Sensor用）
    pub fn new_sensor(
        project_id: [u8; 16],
        device_id: [u8; 16],
        body: SensorDataBody,
        time: i64,
    ) -> Self {
        Self {
            header: MessageHeader {
                project_id,
                device_id,
                r#type: BodyType::Sensor,
                version: VERSION,
                _padding: [0; 6],
                time,
            },
            raw_body: body.as_bytes().to_vec(),
        }
    }

    /// 2. 送信用のコンストラクタ（Media用: 固定長メタデータ + 可変長データ）
    pub fn new_media(
        project_id: [u8; 16],
        device_id: [u8; 16],
        meta: MediaDataMeta,
        mut data: Vec<u8>,
        time: i64,
    ) -> Self {
        let mut raw_body = meta.as_bytes().to_vec();
        raw_body.append(&mut data); // メタデータの後ろに可変長データを結合

        Self {
            header: MessageHeader {
                project_id,
                device_id,
                r#type: BodyType::Media,
                version: VERSION,
                _padding: [0; 6],
                time,
            },
            raw_body,
        }
    }

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

    //
    // 以下、必要な時に生データから各構造体へキャストする抽出メソッド
    //

    /// Sensorデータとして解釈を試みる
    pub fn extract_sensor_body(&self) -> Option<SensorDataBody> {
        if self.header.r#type != BodyType::Sensor {
            return None;
        }
        // FromBytesが実装されているので、サイズが合えば安全に読み取れる
        SensorDataBody::read_from_bytes(self.raw_body.as_slice()).ok()
    }

    /// Mediaデータとして解釈を試みる（メタデータと、可変長データのスライスを返す）
    pub fn extract_media_data(&self) -> Option<(MediaDataMeta, &[u8])> {
        if self.header.r#type != BodyType::Media {
            return None;
        }

        let meta_size = size_of::<MediaDataMeta>();
        if self.raw_body.len() < meta_size {
            return None; // データが足りない
        }

        let meta = MediaDataMeta::read_from_bytes(&self.raw_body[..meta_size]).ok()?;
        let data = &self.raw_body[meta_size..]; // メタデータ以降のスライス

        Some((meta, data))
    }
}
