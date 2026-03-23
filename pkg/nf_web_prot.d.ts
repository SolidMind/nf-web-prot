/* tslint:disable */
/* eslint-disable */

export class WasmDecodedPacket {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    body_size: number;
    device_id: string;
    interval_ms: number;
    project_id: string;
    state_flag: number;
    time: bigint;
    readonly body: Uint8Array;
    readonly codec: Uint8Array;
    readonly mask_index: Uint8Array;
    readonly mask_white_ratio: Float32Array;
}

export function wasm_deserialize_header_to_json(bytes: Uint8Array): any;

export function wasm_deserialize_packet(bytes: Uint8Array): WasmDecodedPacket;

export function wasm_deserialize_to_json(bytes: Uint8Array): any;

export function wasm_serialize_packet(project_id_str: string, device_id_str: string, time: bigint, interval_ms: number, mask_white_ratio_array: Float32Array, codec_array: Uint8Array, body_size: number, state_flag: number, mask_index_array: Uint8Array, body: Uint8Array): Uint8Array;
