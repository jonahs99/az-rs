/* tslint:disable */
/* eslint-disable */
/**
*/
export class Client {
  free(): void;
/**
* @param {number} n
* @returns {Client}
*/
  static new(n: number): Client;
/**
* @param {number} action
*/
  play(action: number): void;
/**
* @param {number} its
* @returns {number}
*/
  think(its: number): number;
/**
* @returns {number | undefined}
*/
  terminal_value(): number | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_client_free: (a: number) => void;
  readonly client_new: (a: number) => number;
  readonly client_play: (a: number, b: number) => void;
  readonly client_think: (a: number, b: number) => number;
  readonly client_terminal_value: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        