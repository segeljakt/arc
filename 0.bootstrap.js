(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/arc_script_wasm.js":
/*!*********************************!*\
  !*** ../pkg/arc_script_wasm.js ***!
  \*********************************/
/*! exports provided: compile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./arc_script_wasm_bg.wasm */ \"../pkg/arc_script_wasm_bg.wasm\");\n/* harmony import */ var _arc_script_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./arc_script_wasm_bg.js */ \"../pkg/arc_script_wasm_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compile\", function() { return _arc_script_wasm_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compile\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/arc_script_wasm.js?");

/***/ }),

/***/ "../pkg/arc_script_wasm_bg.js":
/*!************************************!*\
  !*** ../pkg/arc_script_wasm_bg.js ***!
  \************************************/
/*! exports provided: compile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compile\", function() { return compile; });\n/* harmony import */ var _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./arc_script_wasm_bg.wasm */ \"../pkg/arc_script_wasm_bg.wasm\");\n\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} source\n* @returns {string}\n*/\nfunction compile(source) {\n    try {\n        const retptr = _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value - 16;\n        _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value = retptr;\n        var ptr0 = passStringToWasm0(source, _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compile\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_export_0\"].value += 16;\n        _arc_script_wasm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/arc_script_wasm_bg.js?");

/***/ }),

/***/ "../pkg/arc_script_wasm_bg.wasm":
/*!**************************************!*\
  !*** ../pkg/arc_script_wasm_bg.wasm ***!
  \**************************************/
/*! exports provided: memory, compile, __wbindgen_export_0, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/arc_script_wasm_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var arc_script_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! arc-script-wasm */ \"../pkg/arc_script_wasm.js\");\n/* harmony import */ var xterm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! xterm */ \"./node_modules/xterm/lib/xterm.js\");\n/* harmony import */ var xterm__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(xterm__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var xterm_addon_fit__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! xterm-addon-fit */ \"./node_modules/xterm-addon-fit/lib/xterm-addon-fit.js\");\n/* harmony import */ var xterm_addon_fit__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(xterm_addon_fit__WEBPACK_IMPORTED_MODULE_2__);\n\n\n\n\nvar term = new xterm__WEBPACK_IMPORTED_MODULE_1__[\"Terminal\"]({cursorBlink : true, rows : 50});\nvar elem = document.getElementById('terminal');\nterm.open(elem);\nterm.prompt = () => { term.write('\\r\\n$ '); };\nterm.writeln('This is a REPL for the arc-script language.');\nterm.prompt();\nvar curr_line = '';\n\nterm.onKey(e => {\n  const printable = !e.domEvent.altKey && !e.domEvent.altGraphKey &&\n                    !e.domEvent.ctrlKey && !e.domEvent.metaKey;\n\n  // https://keycode.info/\n  if (e.domEvent.keyCode === 13) {\n    if (curr_line != '') {\n      // <Enter>\n      var input = curr_line;\n      curr_line = '';\n      term.writeln(\"\");\n      var msg = '';\n      try {\n        msg = arc_script_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compile\"](input).replace(/\\n/g, '\\n\\r')\n      } catch (err) {\n        msg = err.message\n      };\n      term.write(msg);\n      term.prompt();\n    } else {\n      term.prompt();\n    }\n  } else if (e.domEvent.keyCode === 8) {\n    // <Backspace>\n    if (term._core.buffer.x > 2) {\n      term.write('\\b \\b');\n      curr_line = curr_line.slice(0, -1);\n    }\n  } else if (e.domEvent.ctrlKey && e.domEvent.keyCode == 76) {\n    // <C-L>\n    term.clear()\n  } else if (printable) {\n    // <Key>\n    curr_line += e.key;\n    term.write(e.key);\n  }\n});\n\nconst fitAddon = new xterm_addon_fit__WEBPACK_IMPORTED_MODULE_2__[\"FitAddon\"]();\nterm.loadAddon(fitAddon);\nfitAddon.fit();\nelem.focus();\nterm.focus();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ }),

/***/ "./node_modules/xterm-addon-fit/lib/xterm-addon-fit.js":
/*!*************************************************************!*\
  !*** ./node_modules/xterm-addon-fit/lib/xterm-addon-fit.js ***!
  \*************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("!function(e,t){ true?module.exports=t():undefined}(window,function(){return function(e){var t={};function r(n){if(t[n])return t[n].exports;var o=t[n]={i:n,l:!1,exports:{}};return e[n].call(o.exports,o,o.exports,r),o.l=!0,o.exports}return r.m=e,r.c=t,r.d=function(e,t,n){r.o(e,t)||Object.defineProperty(e,t,{enumerable:!0,get:n})},r.r=function(e){\"undefined\"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:\"Module\"}),Object.defineProperty(e,\"__esModule\",{value:!0})},r.t=function(e,t){if(1&t&&(e=r(e)),8&t)return e;if(4&t&&\"object\"==typeof e&&e&&e.__esModule)return e;var n=Object.create(null);if(r.r(n),Object.defineProperty(n,\"default\",{enumerable:!0,value:e}),2&t&&\"string\"!=typeof e)for(var o in e)r.d(n,o,function(t){return e[t]}.bind(null,o));return n},r.n=function(e){var t=e&&e.__esModule?function(){return e.default}:function(){return e};return r.d(t,\"a\",t),t},r.o=function(e,t){return Object.prototype.hasOwnProperty.call(e,t)},r.p=\"\",r(r.s=0)}([function(e,t,r){\"use strict\";Object.defineProperty(t,\"__esModule\",{value:!0});var n=function(){function e(){}return e.prototype.activate=function(e){this._terminal=e},e.prototype.dispose=function(){},e.prototype.fit=function(){var e=this.proposeDimensions();if(e&&this._terminal){var t=this._terminal._core;this._terminal.rows===e.rows&&this._terminal.cols===e.cols||(t._renderService.clear(),this._terminal.resize(e.cols,e.rows))}},e.prototype.proposeDimensions=function(){if(this._terminal&&this._terminal.element&&this._terminal.element.parentElement){var e=this._terminal._core,t=window.getComputedStyle(this._terminal.element.parentElement),r=parseInt(t.getPropertyValue(\"height\")),n=Math.max(0,parseInt(t.getPropertyValue(\"width\"))),o=window.getComputedStyle(this._terminal.element),i=r-(parseInt(o.getPropertyValue(\"padding-top\"))+parseInt(o.getPropertyValue(\"padding-bottom\"))),a=n-(parseInt(o.getPropertyValue(\"padding-right\"))+parseInt(o.getPropertyValue(\"padding-left\")))-e.viewport.scrollBarWidth;return{cols:Math.max(2,Math.floor(a/e._renderService.dimensions.actualCellWidth)),rows:Math.max(1,Math.floor(i/e._renderService.dimensions.actualCellHeight))}}},e}();t.FitAddon=n}])});\n//# sourceMappingURL=xterm-addon-fit.js.map\n\n//# sourceURL=webpack:///./node_modules/xterm-addon-fit/lib/xterm-addon-fit.js?");

/***/ })

}]);