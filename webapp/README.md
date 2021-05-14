# Build

`libpxlr.js`, replace `input = new URL('libpxlr_bg.wasm', import.meta.url);` with `input = new URL('./libpxlr_bg.wasm', 'http://example.com/');`
Move `libpxlr_bg.wasm` to `dist/assets`
