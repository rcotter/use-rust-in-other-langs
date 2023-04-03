import * as wasm from "./calculations_bg.wasm";
import { __wbg_set_wasm } from "./calculations_bg.js";
__wbg_set_wasm(wasm);
export * from "./calculations_bg.js";
