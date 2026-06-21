use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{VERSION, message::{BodyType, MessageHeader, RawMessage}};
#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct MediaDataMeta {
    pub codec: [u8; 4],
    pub data_length: u32,
}
impl RawMessage {
     /// 送信用のコンストラクタ（Media用: 固定長メタデータ + 可変長データ）
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
