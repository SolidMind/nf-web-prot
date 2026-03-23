/* @ts-self-types="./nf_web_prot.d.ts" */

import * as wasm from "./nf_web_prot_bg.wasm";
import { __wbg_set_wasm } from "./nf_web_prot_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    WasmDecodedPacket, wasm_deserialize_header_to_json, wasm_deserialize_packet, wasm_deserialize_to_json, wasm_serialize_packet
} from "./nf_web_prot_bg.js";
