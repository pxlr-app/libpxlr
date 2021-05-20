#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{
// 	console, File, FileSystemCreateWritableOptions, FileSystemFileHandle,
// 	FileSystemHandlePermissionDescriptor, FileSystemPermissionMode, FileSystemWritableFileStream,
// };

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(not(target_arch = "wasm32"))]
mod native;

mod document;
mod editor;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	Ok(())
}

// #[cfg(not(feature = "wasm"))]
// #[no_mangle]
// pub extern "C" fn pxlr_hello_world(ptr: *const u8, len: usize) -> *mut String {
// 	let name = unsafe {
// 		let slice = ::std::slice::from_raw_parts(ptr, len);
// 		::std::str::from_utf8_unchecked(slice)
// 	};
// 	let mut name = name.to_string();
// 	name.push_str(" world!");
// 	Box::into_raw(Box::new(name))
// }

// #[cfg(feature = "wasm")]
// #[wasm_bindgen]
// pub fn pxlr_hello_world(mut word: String) -> String {
// 	word.push_str(" world");
// 	word
// }

// #[cfg(feature = "wasm")]
// #[wasm_bindgen]
// pub async fn pxlr_print_file(handle: FileSystemFileHandle) -> Result<(), JsValue> {
// 	use document_core::Node;
// 	// Request permission to user
// 	let state = JsFuture::from(handle.request_permission_with_descriptor(
// 		FileSystemHandlePermissionDescriptor::new().mode(FileSystemPermissionMode::Read),
// 	))
// 	.await?;
// 	// If permission granted
// 	console::log_2(&JsValue::from_str("Permission:"), &state);
// 	if state == JsValue::from_str("granted") {
// 		// Retrieve File from handle
// 		let file: File = JsFuture::from(handle.get_file()).await?.into();
// 		// Create FileReader
// 		let mut reader = web::FileReader::new(file);

// 		// Read document
// 		let document = document_file::File::read(&mut reader)
// 			.await
// 			.expect("Could not read file");
// 		// Retrieve root node
// 		let root = document
// 			.get_root_node(&mut reader)
// 			.await
// 			.expect("Could not get root node");

// 		console::log_3(
// 			&JsValue::from_str("Root:"),
// 			&JsValue::from_str(&root.id().to_string()),
// 			&JsValue::from_str(root.name()),
// 		);
// 	}
// 	Ok(())
// }

// #[cfg(feature = "wasm")]
// #[wasm_bindgen]
// pub async fn pxlr_write_file(handle: FileSystemFileHandle) -> Result<(), JsValue> {
// 	// Request permission to user
// 	let state = JsFuture::from(handle.request_permission_with_descriptor(
// 		FileSystemHandlePermissionDescriptor::new().mode(FileSystemPermissionMode::Readwrite),
// 	))
// 	.await?;
// 	// If permission granted
// 	console::log_2(&JsValue::from_str("Permission:"), &state);
// 	if state == JsValue::from_str("granted") {
// 		// Retrieve File from handle
// 		let file: File = JsFuture::from(handle.get_file()).await?.into();
// 		let size = file.size() as u64;
// 		let stream: FileSystemWritableFileStream =
// 			JsFuture::from(handle.create_writable_with_options(
// 				&FileSystemCreateWritableOptions::new().keep_existing_data(true),
// 			))
// 			.await?
// 			.into();
// 		let mut writer = web::FileWriter::new(stream, size);

// 		// Create dummy doc
// 		let mut document = document_file::File::default();
// 		let note = document_core::Note::new("My note", (0, 0), "");
// 		let root = Arc::new(document_core::NodeType::Note(note));
// 		document.set_root_node(root.clone());

// 		// Write document
// 		document
// 			.append(&mut writer)
// 			.await
// 			.expect("Could not write document");

// 		// todo!("writer.close https://docs.rs/async-std/1.9.0/async_std/fs/struct.File.html, https://github.com/async-rs/async-std/blob/master/src/fs/file.rs#L220");
// 		writer.close().await;

// 		console::log_1(&JsValue::from_str("Blep:"));
// 	}
// 	Ok(())
// }
