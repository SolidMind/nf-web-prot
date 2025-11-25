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
use zerocopy::IntoBytes;

use crate::message::{MessageHeader, PacketProtocol};

impl PacketProtocol {
    pub fn serialize(header: &MessageHeader, body: &[u8]) -> Vec<u8> {
        // ヘッダーとボディの合計サイズで領域確保
        let mut packet = Vec::with_capacity(size_of::<MessageHeader>() + body.len());

        // zerocopy: ヘッダーをバイト列として追加 はゼロコスト
        packet.extend_from_slice(header.as_bytes());

        // ボディを追加
        packet.extend_from_slice(body);

        packet
    }
}
