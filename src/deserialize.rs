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
use crate::message::{MessageHeader, PacketProtocol};
use zerocopy::Ref;

impl PacketProtocol {
    pub fn deserialize(bytes: &[u8]) -> Result<(&MessageHeader, &[u8]), &'static str> {
        let header_len = size_of::<MessageHeader>();

        // データ長がヘッダーサイズ以上あるか確認
        if bytes.len() < header_len {
            return Err("Received data is too short to contain a header");
        }

        // ヘッダー部分とボディ部分にスライスを分割
        let (header_bytes, body_bytes) = bytes.split_at(header_len);

        // zerocopy::Refでヘッダーのパース
        // アラインメントチェック
        // TODO: unwrapを消す
        let header_ref = Ref::<_, MessageHeader>::from_bytes(header_bytes).unwrap();

        // Ref型から&MessageHeaderの参照を取得
        let header = zerocopy::Ref::<&[u8], MessageHeader>::into_ref(header_ref);

        // ボディサイズの整合性チェック
        if header.body_size != body_bytes.len() as u64 {
            return Err("Body size mismatch");
        }

        Ok((header, body_bytes))
    }
}
