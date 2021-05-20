use wasm_bindgen::prelude::*;

mod file;

pub use file::*;

#[wasm_bindgen]
pub async fn pxlr_file_reader_create(
	handle: *mut crate::editor::Editor,
	file_handle: web_sys::FileSystemFileHandle,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
	if handle.is_null() {
		Err(wasm_bindgen::JsValue::from_f64(400.0)) // Editor handle not valid.
	} else {
		let editor = unsafe { &mut *handle };
		let promise = file_handle.request_permission_with_descriptor(
			web_sys::FileSystemHandlePermissionDescriptor::new()
				.mode(web_sys::FileSystemPermissionMode::Read),
		);
		let state = wasm_bindgen_futures::JsFuture::from(promise).await?;
		if state != JsValue::from_str("granted") {
			Err(wasm_bindgen::JsValue::from_f64(401.0)) // Permission not granted to access file.
		} else {
			// Retrieve File from handle
			let file: web_sys::File = wasm_bindgen_futures::JsFuture::from(file_handle.get_file())
				.await?
				.into();
			// Create FileReader
			let reader = crate::web::FileReader::new(file);
			// Return pointer
			let ptr = Box::into_raw(Box::new(reader));
			Ok(wasm_bindgen::JsValue::from_f64(
				ptr as *mut crate::web::FileReader as usize as f64,
			))
		}
	}
}

#[wasm_bindgen]
pub fn pxlr_file_reader_close(file_reader_handle: *mut crate::web::FileReader) {
	if !file_reader_handle.is_null() {
		unsafe {
			Box::from_raw(file_reader_handle);
		}
	}
}

#[wasm_bindgen]
pub async fn pxlr_file_writer_create(
	handle: *mut crate::editor::Editor,
	file_handle: web_sys::FileSystemFileHandle,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
	if handle.is_null() {
		Err(wasm_bindgen::JsValue::from_f64(400.0)) // Editor handle not valid.
	} else {
		let editor = unsafe { &mut *handle };
		let promise = file_handle.request_permission_with_descriptor(
			web_sys::FileSystemHandlePermissionDescriptor::new()
				.mode(web_sys::FileSystemPermissionMode::Read),
		);
		let state = wasm_bindgen_futures::JsFuture::from(promise).await?;
		if state != JsValue::from_str("granted") {
			Err(wasm_bindgen::JsValue::from_f64(401.0)) // Permission not granted to access file.
		} else {
			// Retrieve File from handle
			let file: web_sys::File = wasm_bindgen_futures::JsFuture::from(file_handle.get_file())
				.await?
				.into();
			// Retrieve File size
			let size = file.size() as u64;
			// Create WritableStream
			let stream: web_sys::FileSystemWritableFileStream =
				wasm_bindgen_futures::JsFuture::from(file_handle.create_writable_with_options(
					&web_sys::FileSystemCreateWritableOptions::new().keep_existing_data(true),
				))
				.await?
				.into();
			// Create FileWriter
			let writer = crate::web::FileWriter::new(stream, size);
			// Return pointer
			let ptr = Box::into_raw(Box::new(writer));
			Ok(wasm_bindgen::JsValue::from_f64(
				ptr as *mut crate::web::FileWriter as usize as f64,
			))
		}
	}
}

#[wasm_bindgen]
pub async fn pxlr_file_writer_close(file_writer_handle: *mut crate::web::FileWriter) {
	if !file_writer_handle.is_null() {
		let mut writer = unsafe { Box::from_raw(file_writer_handle) };
		writer.close().await;
	}
}
