/* tslint:disable */
/* eslint-disable */
/**
* @param {Entry} item
*/
export function add_note(item: Entry): void;
/**
*/
export class Entry {
  free(): void;
/**
*/
  date_created: bigint;
/**
*/
  date_modified: bigint;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_entry_free: (a: number) => void;
  readonly __wbg_get_entry_date_created: (a: number) => number;
  readonly __wbg_set_entry_date_created: (a: number, b: number) => void;
  readonly __wbg_get_entry_date_modified: (a: number) => number;
  readonly __wbg_set_entry_date_modified: (a: number, b: number) => void;
  readonly add_note: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
