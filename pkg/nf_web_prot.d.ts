/* tslint:disable */
/* eslint-disable */

export class WasmDecodedPacket {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    body_size: number;
    device_id_hi: bigint;
    device_id_lo: bigint;
    interval_ms: number;
    mask_index: number;
    mask_white_ratio: number;
    project_id_hi: bigint;
    project_id_lo: bigint;
    state_flag: number;
    time: bigint;
    readonly body: Uint8Array;
    readonly codec: Uint8Array;
}

export function wasm_deserialize_packet(bytes: Uint8Array): WasmDecodedPacket;

export function wasm_serialize_packet(project_id_hi: bigint, project_id_lo: bigint, device_id_hi: bigint, device_id_ho: bigint, time: bigint, interval_ms: number, mask_white_ratio: number, codec_array: Uint8Array, body_size: number, state_flag: number, mask_index: number, body: Uint8Array): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_serialize_packet: (a: bigint, b: bigint, c: bigint, d: bigint, e: bigint, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => [number, number, number, number];
    readonly __wbg_wasmdecodedpacket_free: (a: number, b: number) => void;
    readonly __wbg_get_wasmdecodedpacket_project_id_hi: (a: number) => bigint;
    readonly __wbg_set_wasmdecodedpacket_project_id_hi: (a: number, b: bigint) => void;
    readonly __wbg_get_wasmdecodedpacket_project_id_lo: (a: number) => bigint;
    readonly __wbg_set_wasmdecodedpacket_project_id_lo: (a: number, b: bigint) => void;
    readonly __wbg_get_wasmdecodedpacket_device_id_hi: (a: number) => bigint;
    readonly __wbg_set_wasmdecodedpacket_device_id_hi: (a: number, b: bigint) => void;
    readonly __wbg_get_wasmdecodedpacket_device_id_lo: (a: number) => bigint;
    readonly __wbg_set_wasmdecodedpacket_device_id_lo: (a: number, b: bigint) => void;
    readonly __wbg_get_wasmdecodedpacket_time: (a: number) => bigint;
    readonly __wbg_set_wasmdecodedpacket_time: (a: number, b: bigint) => void;
    readonly __wbg_get_wasmdecodedpacket_interval_ms: (a: number) => number;
    readonly __wbg_set_wasmdecodedpacket_interval_ms: (a: number, b: number) => void;
    readonly __wbg_get_wasmdecodedpacket_mask_white_ratio: (a: number) => number;
    readonly __wbg_set_wasmdecodedpacket_mask_white_ratio: (a: number, b: number) => void;
    readonly __wbg_get_wasmdecodedpacket_state_flag: (a: number) => number;
    readonly __wbg_set_wasmdecodedpacket_state_flag: (a: number, b: number) => void;
    readonly __wbg_get_wasmdecodedpacket_mask_index: (a: number) => number;
    readonly __wbg_set_wasmdecodedpacket_mask_index: (a: number, b: number) => void;
    readonly __wbg_get_wasmdecodedpacket_body_size: (a: number) => number;
    readonly __wbg_set_wasmdecodedpacket_body_size: (a: number, b: number) => void;
    readonly wasmdecodedpacket_codec: (a: number) => [number, number];
    readonly wasmdecodedpacket_body: (a: number) => [number, number];
    readonly wasm_deserialize_packet: (a: number, b: number) => [number, number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
