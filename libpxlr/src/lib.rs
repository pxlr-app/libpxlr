use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	// Your code goes here!
	unsafe {
		console::log_1(&JsValue::from_str("Hello world!"));
	}

	Ok(())
}

#[cfg(not(feature = "wasm"))]
#[no_mangle]
pub extern "C" fn pxlr_hello_world(ptr: *const u8, len: usize) -> *mut String {
	let name = unsafe {
		let slice = ::std::slice::from_raw_parts(ptr, len);
		::std::str::from_utf8_unchecked(slice)
	};
	let mut name = name.to_string();
	name.push_str(" world!");
	Box::into_raw(Box::new(name))
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn pxlr_hello_world(mut word: String) -> String {
	word.push_str(" world");
	word
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn pxlr_print_file() {
	unimplemented!()
}
