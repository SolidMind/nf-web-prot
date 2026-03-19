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
use core::ops::{BitAnd, BitOr};
use std::fmt::{self, Debug};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};
pub struct PacketProtocol;

#[repr(u8)]
pub enum PacketType {
    Request,
    Response,
    Notification,
}

/// StatusFlagsのorはCatchMoveとSilenceにTranseEdge、TranseDiff、TranseCon、OrderPass、OrderErrorの何れかが可能。
/// TODO: ロジックとしてorを制限する。
/// TODO CPUに優しくするために4バイトや8バイト区切りでアクセスに対応。
///　TODO codecのfmt
#[derive(Debug, FromBytes, IntoBytes, PartialEq, Immutable, KnownLayout)]
#[repr(C)]
pub struct MessageHeader {
    pub version: u8, // 1 byte  (Offset 58)
    pub _reserved1: [u8; 1],
    pub project_id: [u8; 16], // 16 bytes (Offset 0)
    pub device_id: [u8; 16],  // 16 bytes (Offset 16) <- u32から変更

    pub state: StatusFlags,         // 2 bytes (Offset 56)
    pub mask_index: [u8; 8],        // 1 byte  (Offset 59)
    pub mask_white_ratio: [f32; 8], // 4 bytes (Offset 44)
    pub interval_ms: u32,           // 4 bytes (Offset 40)

    pub codec: [u8; 4], // 4 bytes (Offset 48)
    pub body_size: u32, // 4 bytes (Offset 52)
    pub time: i64,      // 8 bytes (Offset 32)
}

impl MessageHeader {
    pub fn new(
        project_id: [u8; 16],
        device_id: [u8; 16], // <- u32から変更
        time: i64,
        state: StatusFlags,
        mask_index: [u8; 8],
        mask_white_ratio: [f32; 8],
        interval_ms: u32,
        codec: [u8; 4],
        body_size: u32,
    ) -> Self {
        Self {
            version: crate::CURRENT_PROTOCOL_VERSION,
            project_id,
            device_id,
            time,
            state,
            interval_ms,
            mask_white_ratio,
            codec,
            mask_index,
            body_size,
            _reserved1: [0; 1],
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable)]
pub struct StatusFlags(pub u16);

impl StatusFlags {
    pub const CATCH_MOVE: Self = Self(1 << 0);
    pub const SILENCE: Self = Self(1 << 1);
    pub const REST_SENSING: Self = Self(1 << 2);
    pub const AWAKE_SENSING: Self = Self(1 << 3);
    pub const START_DEVICE: Self = Self(1 << 4);
    pub const REST_PUSH: Self = Self(1 << 5);

    pub const ORDER_PASS: Self = Self(1 << 6);
    pub const ORDER_ERROR: Self = Self(1 << 7);
    pub const TRANSE_EDGE: Self = Self(1 << 8);
    pub const TRANSE_DIFF: Self = Self(1 << 9);
    pub const TRANSE_CON: Self = Self(1 << 10);
    pub const TRY_RESTART: Self = Self(1 << 11);

    pub const UNWORK_CAMERA: Self = Self(1 << 12);
    pub const DISCONNECT: Self = Self(1 << 13);
    pub const CONNECT: Self = Self(1 << 14);
}
impl BitOr for StatusFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        StatusFlags(self.0 | rhs.0)
    }
}

impl BitAnd for StatusFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        StatusFlags(self.0 & rhs.0)
    }
}

impl fmt::Debug for StatusFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = [
            (Self::CATCH_MOVE, "CATCH_MOVE"),
            (Self::SILENCE, "SILENCE"),
            (Self::REST_SENSING, "REST_SENSING"),
            (Self::AWAKE_SENSING, "AWAKE_SENSING"),
            (Self::START_DEVICE, "START_DEVICE"),
            (Self::REST_PUSH, "REST_PUSH"),
            (Self::ORDER_PASS, "ORDER_PASS"),
            (Self::ORDER_ERROR, "ORDER_ERROR"),
            (Self::TRANSE_EDGE, "TRANSE_EDGE"),
            (Self::TRANSE_DIFF, "TRANSE_DIFF"),
            (Self::TRANSE_CON, "TRANSE_CON"),
            (Self::TRY_RESTART, "TRY_RESTART"),
            (Self::UNWORK_CAMERA, "UNWORK_CAMERA"),
        ];

        let active_flags: Vec<&str> = flags
            .iter()
            .filter(|(flag, _)| (self.0 & flag.0) != 0)
            .map(|(_, name)| *name)
            .collect();

        if !active_flags.is_empty() {
            let known_mask = flags.iter().fold(0, |acc, (flag, _)| acc | flag.0);
            let remaining = self.0 & !known_mask;

            let flags_str = active_flags.join(" | ");

            if remaining != 0 {
                write!(f, "StatusFlags({} | {:#x})", flags_str, remaining)
            } else {
                write!(f, "StatusFlags({})", flags_str)
            }
        } else {
            // フラグが何も立っていない、または未知の数値のみの場合
            write!(f, "StatusFlags({:#x})", self.0)
        }
    }
}
