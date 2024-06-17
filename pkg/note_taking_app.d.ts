/* tslint:disable */
/* eslint-disable */
/**
* @param {Entry} entry
*/
export function add_note(entry: Entry): void;
/**
* @param {Entry} entry
* @param {number} key
*/
export function modify_note(entry: Entry, key: number): void;
/**
* @param {number} key
*/
export function remove_note(key: number): void;
/**
* @param {Category} category
* @param {SortBy} sort_by
* @param {Mode} sorting_mode
*/
export function filter_by_mode(category: Category, sort_by: SortBy, sorting_mode: Mode): void;
/**
* @param {bigint} unix_time
* @returns {any}
*/
export function unix_to_utc_date(unix_time: bigint): any;
/**
* @param {bigint} unix_time
* @returns {any}
*/
export function unix_to_utc_plus_one(unix_time: bigint): any;
/**
*/
export enum SortBy {
  Unsorted = 0,
  DateCreated = 1,
  DateModified = 2,
  Title = 3,
}
/**
*/
export enum Category {
  All = 0,
  Draft = 1,
  InProgress = 2,
  Cancelled = 3,
  Done = 4,
}
/**
*/
export enum Mode {
  Ascending = 0,
  Descending = 1,
}
/**
*/
export class Entry {
  free(): void;
/**
* @returns {Entry}
*/
  static new(): Entry;
/**
* @param {Category} category
* @param {string} title
* @param {string} message
* @param {(string)[]} tags
*/
  constructor(category: Category, title: string, message: string, tags: (string)[]);
/**
*/
  date_created: bigint;
/**
*/
  date_modified: bigint;
/**
*/
  entry: Entry;
/**
*/
  readonly get_category: Category;
/**
*/
  readonly title: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly add_note: (a: number) => void;
  readonly modify_note: (a: number, b: number) => void;
  readonly remove_note: (a: number) => void;
  readonly filter_by_mode: (a: number, b: number, c: number) => void;
  readonly unix_to_utc_date: (a: number) => number;
  readonly unix_to_utc_plus_one: (a: number) => number;
  readonly __wbg_entry_free: (a: number) => void;
  readonly __wbg_get_entry_date_created: (a: number) => number;
  readonly __wbg_set_entry_date_created: (a: number, b: number) => void;
  readonly __wbg_get_entry_date_modified: (a: number) => number;
  readonly __wbg_set_entry_date_modified: (a: number, b: number) => void;
  readonly entry_new: () => number;
  readonly entry_set_entry: (a: number, b: number) => void;
  readonly entry_from: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly entry_get_category: (a: number) => number;
  readonly entry_title: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
