use async_std::io::prelude::SeekExt;
use std::pin::Pin;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
	console, File, FileSystemFileHandle, FileSystemHandlePermissionDescriptor,
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
	console::log_1(&JsValue::from_str("Hello world!"));

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
	use async_std::io::ReadExt;
	// Request permission to user
	let state = JsFuture::from(handle.request_permission_with_descriptor(
		FileSystemHandlePermissionDescriptor::new().mode(FileSystemPermissionMode::Read),
	))
	.await?;
	// If permission granted
	console::log_2(&JsValue::from_str("Permission:"), &state);
	if state == JsValue::from_str("granted") {
		// Retrieve File from handle
		let file: File = JsFuture::from(handle.get_file()).await?.into();
		// Create FileReader
		let mut reader = FileReader::new(file);
		// Retrieve first 10 bytes
		let mut buffer: Vec<u8> = vec![0u8; 5];
		let _ = reader.seek(async_std::io::SeekFrom::End(-5)).await;
		let _ = reader.read_exact(&mut buffer).await;
		// Decode bytes to UTF8 string
		let decoder = TextDecoder::new().unwrap();
		if let Ok(content) = decoder.decode_with_u8_array(&mut buffer[..]) {
			console::log_2(&JsValue::from_str("Content:"), &JsValue::from_str(&content));
		}
	}
	Ok(())
}

pub struct FileReader {
	file: File,
	offset: i64,
	future: Option<JsFuture>,
}

impl FileReader {
	pub fn new(file: File) -> Self {
		Self {
			file,
			offset: 0,
			future: None,
		}
	}
}

impl async_std::io::Seek for FileReader {
	fn poll_seek(
		self: std::pin::Pin<&mut Self>,
		_cx: &mut async_std::task::Context<'_>,
		pos: async_std::io::SeekFrom,
	) -> async_std::task::Poll<async_std::io::Result<u64>> {
		let self_mut = self.get_mut();
		let new_offset = match pos {
			async_std::io::SeekFrom::Current(pos) => self_mut.offset + pos,
			async_std::io::SeekFrom::End(pos) => pos as i64,
			async_std::io::SeekFrom::Start(pos) => pos as i64,
		};
		// console::log_2(&JsValue::from_str("Offset:"), &JsValue::from_f64(new_offset as f64));
		self_mut.offset = new_offset;
		async_std::task::Poll::Ready(Ok(new_offset as u64))
	}
}

impl async_std::io::Read for FileReader {
	fn poll_read(
		self: std::pin::Pin<&mut Self>,
		cx: &mut async_std::task::Context<'_>,
		buf: &mut [u8],
	) -> async_std::task::Poll<async_std::io::Result<usize>> {
		let len = buf.len();
		let mut_self = self.get_mut();
		let future = mut_self.future.take();
		match future {
			None => {
				// console::log_3(&JsValue::from_str("Slice:"), &JsValue::from_f64(mut_self.offset as f64), &JsValue::from_f64((mut_self.offset + len as i64) as f64));
				let start: i64 = mut_self.offset;
				let end: i64 = mut_self.offset + len as i64;
				let blob: web_sys::Blob = if end != 0 {
					mut_self
						.file
						.slice_with_i32_and_i32(start as i32, end as i32)
						.unwrap()
				} else {
					mut_self.file.slice_with_i32(start as i32).unwrap()
				};
				let mut buffer = JsFuture::from(blob.array_buffer());
				let pinned = Pin::new(&mut buffer);
				// Poll future to initiate waker
				let _ = std::future::Future::poll(pinned, cx);
				mut_self.future.replace(buffer);
				async_std::task::Poll::Pending
			}
			Some(mut js_future) => {
				let pinned = Pin::new(&mut js_future);
				match std::future::Future::poll(pinned, cx) {
					async_std::task::Poll::Pending => {
						mut_self.future.replace(js_future);
						async_std::task::Poll::Pending
					}
					async_std::task::Poll::Ready(res) => {
						let buffer = js_sys::Uint8Array::new(&res.unwrap());
						// console::log_3(&JsValue::from_str("Bytes:"), &JsValue::from_f64(buffer.byte_length() as f64), &JsValue::from_f64(len as f64));
						buffer.copy_to(buf);
						async_std::task::Poll::Ready(Ok(len))
					}
				}
			}
		}
	}
}
