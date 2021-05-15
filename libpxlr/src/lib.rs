// mod downcast;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
	console, Blob, File, FileSystemFileHandle, FileSystemHandlePermissionDescriptor,
	FileSystemPermissionMode, TextDecoder,
};
// use downcast::downcast_jsvalue;

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
pub async fn pxlr_print_file(handle: FileSystemFileHandle) -> Result<(), JsValue> {
	let state = JsFuture::from(handle.request_permission_with_descriptor(
		FileSystemHandlePermissionDescriptor::new().mode(FileSystemPermissionMode::Read),
	))
	.await?;
	unsafe {
		console::log_2(&JsValue::from_str("Permission:"), &state);
	}
	if state == JsValue::from_str("granted") {
		let file: File = JsFuture::from(handle.get_file()).await?.into();
		// let buffer = file.slice_with_i32_and_i32(0, 10).unwrap();
		// let text = JsFuture::from(buffer.text()).await?;
		// unsafe { console::log_2(&JsValue::from_str("Content:"), &text); }
		let decoder = TextDecoder::new().unwrap();
		let buffer = JsFuture::from(file.array_buffer()).await?;
		if let Ok(content) = decoder.decode_with_buffer_source(&buffer.into()) {
			unsafe {
				console::log_2(&JsValue::from_str("Content:"), &JsValue::from_str(&content));
			}
		}
	}
	Ok(())
}
