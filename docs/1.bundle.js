(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./wasm sync recursive":
/*!*******************!*\
  !*** ./wasm sync ***!
  \*******************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("function webpackEmptyContext(req) {\n\tvar e = new Error(\"Cannot find module '\" + req + \"'\");\n\te.code = 'MODULE_NOT_FOUND';\n\tthrow e;\n}\nwebpackEmptyContext.keys = function() { return []; };\nwebpackEmptyContext.resolve = webpackEmptyContext;\nmodule.exports = webpackEmptyContext;\nwebpackEmptyContext.id = \"./wasm sync recursive\";\n\n//# sourceURL=webpack:///./wasm_sync?");

/***/ }),

/***/ "./wasm/tetris.js":
/*!************************!*\
  !*** ./wasm/tetris.js ***!
  \************************/
/*! exports provided: Control, draw, __widl_f_begin_path_CanvasRenderingContext2D, __widl_f_stroke_CanvasRenderingContext2D, __widl_f_set_stroke_style_CanvasRenderingContext2D, __widl_f_set_fill_style_CanvasRenderingContext2D, __widl_f_line_to_CanvasRenderingContext2D, __widl_f_move_to_CanvasRenderingContext2D, __widl_f_fill_rect_CanvasRenderingContext2D, __wbg_new_baf10398b0d0c64d, __wbg_call_173f04c850a68d5f, __wbg_self_58232ab37cbe6608, __wbg_crypto_329b714d7e7d321d, __wbg_getRandomValues_2f960218fce3a102, __wbg_getRandomValues_5581e85fc6616df6, __wbg_require_4a70cbfd3adc73a8, __wbg_randomFillSync_355c3fcfa754fa4e, Tetris, __wbindgen_object_drop_ref, __wbindgen_string_new, __wbindgen_is_undefined, __wbindgen_jsval_eq, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Control\", function() { return Control; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"draw\", function() { return draw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_begin_path_CanvasRenderingContext2D\", function() { return __widl_f_begin_path_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_stroke_CanvasRenderingContext2D\", function() { return __widl_f_stroke_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_stroke_style_CanvasRenderingContext2D\", function() { return __widl_f_set_stroke_style_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_set_fill_style_CanvasRenderingContext2D\", function() { return __widl_f_set_fill_style_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_line_to_CanvasRenderingContext2D\", function() { return __widl_f_line_to_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_move_to_CanvasRenderingContext2D\", function() { return __widl_f_move_to_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__widl_f_fill_rect_CanvasRenderingContext2D\", function() { return __widl_f_fill_rect_CanvasRenderingContext2D; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_baf10398b0d0c64d\", function() { return __wbg_new_baf10398b0d0c64d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_173f04c850a68d5f\", function() { return __wbg_call_173f04c850a68d5f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_58232ab37cbe6608\", function() { return __wbg_self_58232ab37cbe6608; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_329b714d7e7d321d\", function() { return __wbg_crypto_329b714d7e7d321d; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_2f960218fce3a102\", function() { return __wbg_getRandomValues_2f960218fce3a102; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_5581e85fc6616df6\", function() { return __wbg_getRandomValues_5581e85fc6616df6; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_4a70cbfd3adc73a8\", function() { return __wbg_require_4a70cbfd3adc73a8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_355c3fcfa754fa4e\", function() { return __wbg_randomFillSync_355c3fcfa754fa4e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Tetris\", function() { return Tetris; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_string_new\", function() { return __wbindgen_string_new; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_jsval_eq\", function() { return __wbindgen_jsval_eq; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _tetris_bg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./tetris_bg */ \"./wasm/tetris_bg.wasm\");\n/* tslint:disable */\n\n\n/**\n*/\nconst Control = Object.freeze({ MoveLeft:0,MoveRight:1,MoveDown:2,MoveBottom:3,RotateLeft:4,RotateRight:5, });\n\nconst heap = new Array(32);\n\nheap.fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nlet stack_pointer = 32;\n\nfunction addBorrowedObject(obj) {\n    if (stack_pointer == 1) throw new Error('out of js stack');\n    heap[--stack_pointer] = obj;\n    return stack_pointer;\n}\n/**\n* @param {Tetris} arg0\n* @param {any} arg1\n* @returns {void}\n*/\nfunction draw(arg0, arg1) {\n    try {\n        return _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"draw\"](arg0.ptr, addBorrowedObject(arg1));\n\n    } finally {\n        heap[stack_pointer++] = undefined;\n\n    }\n\n}\n\nfunction getObject(idx) { return heap[idx]; }\n\nconst __widl_f_begin_path_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.beginPath || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.beginPath does not exist`);\n};\n\nfunction __widl_f_begin_path_CanvasRenderingContext2D(arg0) {\n    __widl_f_begin_path_CanvasRenderingContext2D_target.call(getObject(arg0));\n}\n\nconst __widl_f_stroke_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.stroke || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.stroke does not exist`);\n};\n\nfunction __widl_f_stroke_CanvasRenderingContext2D(arg0) {\n    __widl_f_stroke_CanvasRenderingContext2D_target.call(getObject(arg0));\n}\n\nfunction GetOwnOrInheritedPropertyDescriptor(obj, id) {\n    while (obj) {\n        let desc = Object.getOwnPropertyDescriptor(obj, id);\n        if (desc) return desc;\n        obj = Object.getPrototypeOf(obj);\n    }\nreturn {}\n}\n\nconst __widl_f_set_stroke_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'strokeStyle').set || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.strokeStyle does not exist`);\n};\n\nfunction __widl_f_set_stroke_style_CanvasRenderingContext2D(arg0, arg1) {\n    __widl_f_set_stroke_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));\n}\n\nconst __widl_f_set_fill_style_CanvasRenderingContext2D_target = GetOwnOrInheritedPropertyDescriptor(typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype, 'fillStyle').set || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillStyle does not exist`);\n};\n\nfunction __widl_f_set_fill_style_CanvasRenderingContext2D(arg0, arg1) {\n    __widl_f_set_fill_style_CanvasRenderingContext2D_target.call(getObject(arg0), getObject(arg1));\n}\n\nconst __widl_f_line_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.lineTo || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.lineTo does not exist`);\n};\n\nfunction __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {\n    __widl_f_line_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);\n}\n\nconst __widl_f_move_to_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.moveTo || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.moveTo does not exist`);\n};\n\nfunction __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {\n    __widl_f_move_to_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2);\n}\n\nconst __widl_f_fill_rect_CanvasRenderingContext2D_target = typeof CanvasRenderingContext2D === 'undefined' ? null : CanvasRenderingContext2D.prototype.fillRect || function() {\n    throw new Error(`wasm-bindgen: CanvasRenderingContext2D.fillRect does not exist`);\n};\n\nfunction __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {\n    __widl_f_fill_rect_CanvasRenderingContext2D_target.call(getObject(arg0), arg1, arg2, arg3, arg4);\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? __webpack_require__(/*! util */ \"./node_modules/util/util.js\").TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8');\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nfunction getStringFromWasm(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));\n}\n\nlet heap_next = heap.length;\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction __wbg_new_baf10398b0d0c64d(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    return addHeapObject(new Function(varg0));\n}\n\nfunction __wbg_call_173f04c850a68d5f(arg0, arg1) {\n    return addHeapObject(getObject(arg0).call(getObject(arg1)));\n}\n\nfunction __wbg_self_58232ab37cbe6608(arg0) {\n    return addHeapObject(getObject(arg0).self);\n}\n\nfunction __wbg_crypto_329b714d7e7d321d(arg0) {\n    return addHeapObject(getObject(arg0).crypto);\n}\n\nfunction __wbg_getRandomValues_2f960218fce3a102(arg0) {\n    return addHeapObject(getObject(arg0).getRandomValues);\n}\n\nfunction getArrayU8FromWasm(ptr, len) {\n    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction __wbg_getRandomValues_5581e85fc6616df6(arg0, arg1, arg2) {\n    let varg1 = getArrayU8FromWasm(arg1, arg2);\n    getObject(arg0).getRandomValues(varg1);\n}\n\nfunction __wbg_require_4a70cbfd3adc73a8(arg0, arg1) {\n    let varg0 = getStringFromWasm(arg0, arg1);\n    return addHeapObject(__webpack_require__(\"./wasm sync recursive\")(varg0));\n}\n\nfunction __wbg_randomFillSync_355c3fcfa754fa4e(arg0, arg1, arg2) {\n    let varg1 = getArrayU8FromWasm(arg1, arg2);\n    getObject(arg0).randomFillSync(varg1);\n}\n\nfunction freeTetris(ptr) {\n\n    _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_tetris_free\"](ptr);\n}\n/**\n*/\nclass Tetris {\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n        freeTetris(ptr);\n    }\n\n    /**\n    * @returns {}\n    */\n    constructor() {\n        this.ptr = _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"tetris_new\"]();\n    }\n    /**\n    * @param {number} arg0\n    * @returns {void}\n    */\n    update(arg0) {\n        return _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"tetris_update\"](this.ptr, arg0);\n    }\n    /**\n    * @param {number} arg0\n    * @returns {void}\n    */\n    enqueue_control(arg0) {\n        return _tetris_bg__WEBPACK_IMPORTED_MODULE_0__[\"tetris_enqueue_control\"](this.ptr, arg0);\n    }\n}\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction __wbindgen_object_drop_ref(i) { dropObject(i); }\n\nfunction __wbindgen_string_new(p, l) {\n    return addHeapObject(getStringFromWasm(p, l));\n}\n\nfunction __wbindgen_is_undefined(idx) {\n    return getObject(idx) === undefined ? 1 : 0;\n}\n\nfunction __wbindgen_jsval_eq(a, b) {\n    return getObject(a) === getObject(b) ? 1 : 0;\n}\n\nfunction __wbindgen_throw(ptr, len) {\n    throw new Error(getStringFromWasm(ptr, len));\n}\n\n\n\n//# sourceURL=webpack:///./wasm/tetris.js?");

/***/ }),

/***/ "./wasm/tetris_bg.wasm":
/*!*****************************!*\
  !*** ./wasm/tetris_bg.wasm ***!
  \*****************************/
/*! exports provided: memory, __wbg_tetris_free, tetris_new, tetris_update, tetris_enqueue_control, draw */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./tetris */ \"./wasm/tetris.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./wasm/tetris_bg.wasm?");

/***/ })

}]);