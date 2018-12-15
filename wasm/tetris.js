/* tslint:disable */
import * as wasm from './tetris_bg';

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbg_log_d74d17cde7c63de5(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    console.log(varg0);
}
/**
*/
export const Control = Object.freeze({ MoveLeft:0,MoveRight:1,RotateLeft:2,RotateRight:3, });

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {Tetris} arg0
* @param {any} arg1
* @returns {void}
*/
export function draw(arg0, arg1) {
    try {
        return wasm.draw(arg0.ptr, addBorrowedObject(arg1));

    } finally {
        heap[stack_pointer++] = undefined;

    }

}

function getObject(idx) { return heap[idx]; }

const __widl_f_begin_path_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.beginPath || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.beginPath does not exist`);
};

export function __widl_f_begin_path_CanvasRenderingContext2D(arg0) {
    __widl_f_begin_path_CanvasRenderingContext2D_target.call(getObject(arg0));
}

const __widl_f_stroke_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.stroke || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.stroke does not exist`);
};

export function __widl_f_stroke_CanvasRenderingContext2D(arg0) {
    __widl_f_stroke_CanvasRenderingContext2D_target.call(getObject(arg0));
}

function GetOwnOrInheritedPropertyDescriptor(obj, id) {
    while (obj) {
        let desc = Object.getOwnPropertyDescriptor(obj, id);
        if (desc) return desc;
        obj = Object.getPrototypeOf(obj);
    }
return {}
}

const __widl_f_set_stroke_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'strokeStyle').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.strokeStyle does not exist`);
};

export function __widl_f_set_stroke_style_CanvasRenderingContext2D(arg0, arg1) {
    __widl_f_set_stroke_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));
}

const __widl_f_set_fill_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'fillStyle').set || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillStyle does not exist`);
};

export function __widl_f_set_fill_style_CanvasRenderingContext2D(arg0, arg1) {
    __widl_f_set_fill_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));
}

const __widl_f_line_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.lineTo || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineTo does not exist`);
};

export function __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    __widl_f_line_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);
}

const __widl_f_move_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.moveTo || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.moveTo does not exist`);
};

export function __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    __widl_f_move_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);
}

const __widl_f_fill_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillRect || function() {
    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillRect does not exist`);
};

export function __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    __widl_f_fill_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

export function __wbg_new_baf10398b0d0c64d(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

export function __wbg_call_173f04c850a68d5f(arg0, arg1) {
    return addHeapObject(getObject(arg0).call(getObject(arg1)));
}

export function __wbg_self_58232ab37cbe6608(arg0) {
    return addHeapObject(getObject(arg0).self);
}

export function __wbg_crypto_329b714d7e7d321d(arg0) {
    return addHeapObject(getObject(arg0).crypto);
}

export function __wbg_getRandomValues_2f960218fce3a102(arg0) {
    return addHeapObject(getObject(arg0).getRandomValues);
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

export function __wbg_getRandomValues_5581e85fc6616df6(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).getRandomValues(varg1);
}

export function __wbg_require_4a70cbfd3adc73a8(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(require(varg0));
}

export function __wbg_randomFillSync_355c3fcfa754fa4e(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).randomFillSync(varg1);
}

function freeTetris(ptr) {

    wasm.__wbg_tetris_free(ptr);
}
/**
*/
export class Tetris {

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeTetris(ptr);
    }

    /**
    * @returns {}
    */
    constructor() {
        this.ptr = wasm.tetris_new();
    }
    /**
    * @param {number} arg0
    * @returns {void}
    */
    update(arg0) {
        return wasm.tetris_update(this.ptr, arg0);
    }
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

export function __wbindgen_object_drop_ref(i) { dropObject(i); }

export function __wbindgen_string_new(p, l) {
    return addHeapObject(getStringFromWasm(p, l));
}

export function __wbindgen_is_undefined(idx) {
    return getObject(idx) === undefined ? 1 : 0;
}

export function __wbindgen_jsval_eq(a, b) {
    return getObject(a) === getObject(b) ? 1 : 0;
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

