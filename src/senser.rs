use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{VERSION, message::{BodyType, MessageHeader, RawMessage}};

#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout, Clone)]
#[repr(C)]
pub struct SensorDataBody {
    pub temperature: f32,
    pub humidity: f32,
}


impl RawMessage {
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

     /// Sensorデータとして解釈を試みる
    pub fn extract_sensor_body(&self) -> Option<SensorDataBody> {
        if self.header.r#type != BodyType::Sensor {
            return None;
        }
        // FromBytesが実装されているので、サイズが合えば安全に読み取れる
        SensorDataBody::read_from_bytes(self.raw_body.as_slice()).ok()
    }
}