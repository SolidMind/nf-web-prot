/*
MIT License

Copyright (c) 2025 yatoneco

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

pub mod deserialize;
pub mod message;
pub mod serialize;

//TODO: MessageHeaderのフィールドを減らす。フィールドをbodyへ

#[cfg(test)]
mod tests {
    //cargo test  -- --nocapture
    use crate::message::*;

    #[test]
    fn test_serialize_deserialize() {
        // 1. 送信データ作成
        let original_body = b"Hello, Protocol!";
        let header = MessageHeader::new(
            123456789,                                          // project_id
            999,                                                // device_id
            0,                                                  // time
            StatusFlags::CATCH_MOVE | StatusFlags::TRANSE_EDGE, // state
            2,                                                  // mask_index
            0.5,                                                // mask_white_ratio
            1000,                                               // interval_ms:
            *b"jpeg",                                           // codec
            original_body.len() as u64,                         // body_size
        );

        let packet = PacketProtocol::serialize(&header, original_body);

        println!("Packet size: {} bytes", packet.len());

        let result = PacketProtocol::deserialize(&packet);

        assert!(result.is_ok());
        let (received_header, received_body) = result.unwrap();

        // 検証
        assert_eq!(received_header, &header);
        assert_eq!(received_body, original_body);

        println!("Successfully parsed header: {:?}", received_header);
        println!(
            "Body content: {:?}",
            std::str::from_utf8(received_body).unwrap()
        );
    }
}
