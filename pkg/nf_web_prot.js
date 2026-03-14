/* @ts-self-types="./nf_web_prot.d.ts" */

export class WasmDecodedPacket {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmDecodedPacket.prototype);
        obj.__wbg_ptr = ptr;
        WasmDecodedPacketFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDecodedPacketFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdecodedpacket_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get body_size() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_body_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {bigint}
     */
    get device_id_hi() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_device_id_hi(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {bigint}
     */
    get device_id_lo() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_device_id_lo(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {number}
     */
    get interval_ms() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_interval_ms(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get mask_index() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_mask_index(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    get mask_white_ratio() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_mask_white_ratio(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {bigint}
     */
    get project_id_hi() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_project_id_hi(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {bigint}
     */
    get project_id_lo() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_project_id_lo(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
     * @returns {number}
     */
    get state_flag() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_state_flag(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {bigint}
     */
    get time() {
        const ret = wasm.__wbg_get_wasmdecodedpacket_time(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set body_size(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_body_size(this.__wbg_ptr, arg0);
    }
    /**
     * @param {bigint} arg0
     */
    set device_id_hi(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_device_id_hi(this.__wbg_ptr, arg0);
    }
    /**
     * @param {bigint} arg0
     */
    set device_id_lo(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_device_id_lo(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set interval_ms(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_interval_ms(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mask_index(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_mask_index(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set mask_white_ratio(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_mask_white_ratio(this.__wbg_ptr, arg0);
    }
    /**
     * @param {bigint} arg0
     */
    set project_id_hi(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_project_id_hi(this.__wbg_ptr, arg0);
    }
    /**
     * @param {bigint} arg0
     */
    set project_id_lo(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_project_id_lo(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} arg0
     */
    set state_flag(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_state_flag(this.__wbg_ptr, arg0);
    }
    /**
     * @param {bigint} arg0
     */
    set time(arg0) {
        wasm.__wbg_set_wasmdecodedpacket_time(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {Uint8Array}
     */
    get body() {
        const ret = wasm.wasmdecodedpacket_body(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {Uint8Array}
     */
    get codec() {
        const ret = wasm.wasmdecodedpacket_codec(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}
if (Symbol.dispose) WasmDecodedPacket.prototype[Symbol.dispose] = WasmDecodedPacket.prototype.free;

/**
 * @param {Uint8Array} bytes
 * @returns {WasmDecodedPacket}
 */
export function wasm_deserialize_packet(bytes) {
    const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_deserialize_packet(ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return WasmDecodedPacket.__wrap(ret[0]);
}

/**
 * @param {bigint} project_id_hi
 * @param {bigint} project_id_lo
 * @param {bigint} device_id_hi
 * @param {bigint} device_id_ho
 * @param {bigint} time
 * @param {number} interval_ms
 * @param {number} mask_white_ratio
 * @param {Uint8Array} codec_array
 * @param {number} body_size
 * @param {number} state_flag
 * @param {number} mask_index
 * @param {Uint8Array} body
 * @returns {Uint8Array}
 */
export function wasm_serialize_packet(project_id_hi, project_id_lo, device_id_hi, device_id_ho, time, interval_ms, mask_white_ratio, codec_array, body_size, state_flag, mask_index, body) {
    const ptr0 = passArray8ToWasm0(codec_array, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(body, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.wasm_serialize_packet(project_id_hi, project_id_lo, device_id_hi, device_id_ho, time, interval_ms, mask_white_ratio, ptr0, len0, body_size, state_flag, mask_index, ptr1, len1);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./nf_web_prot_bg.js": import0,
    };
}

const WasmDecodedPacketFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdecodedpacket_free(ptr >>> 0, 1));

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('nf_web_prot_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
