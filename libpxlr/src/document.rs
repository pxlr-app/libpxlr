#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn pxlr_document_read(
	file_reader_handle: *mut crate::web::FileReader,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
	if file_reader_handle.is_null() {
		Err(wasm_bindgen::JsValue::from_f64(400.0))
	} else {
		let mut file_reader = unsafe { &mut *file_reader_handle };

		// Read document
		match document_file::File::read(&mut file_reader).await {
			Err(_) => Err(wasm_bindgen::JsValue::from_f64(500.0)),
			Ok(document) => {
				let ptr = Box::into_raw(Box::new(document));
				Ok(wasm_bindgen::JsValue::from_f64(
					ptr as *mut document_file::File as usize as f64,
				))
			}
		}
	}
}

#[cfg(not(target_arch = "wasm32"))]
pub extern "C" fn pxlr_document_read(
	file_reader_handle: *mut async_std::fs::File,
) -> *mut document_file::File {
	if file_reader_handle.is_null() {
		std::ptr::null_mut()
	} else {
		let mut file_reader = unsafe { &mut *file_reader_handle };

		match async_std::task::block_on(document_file::File::read(&mut file_reader)) {
			Err(_) => std::ptr::null_mut(),
			Ok(document) => Box::into_raw(Box::new(document)),
		}
	}
}
