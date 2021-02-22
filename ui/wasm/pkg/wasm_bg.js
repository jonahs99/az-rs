import * as wasm from './wasm_bg.wasm';

/**
* @param {number} x
* @param {number} y
* @returns {number}
*/
export function add(x, y) {
    var ret = wasm.add(x, y);
    return ret;
}

