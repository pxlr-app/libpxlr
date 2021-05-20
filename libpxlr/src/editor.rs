use vek::{geom::Rect, vec::Vec2};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Editor {
	document: Option<document_file::File>,
	viewports: std::collections::HashMap<u32, Viewport>,
}

impl Editor {
	pub fn document(&self) -> &Option<document_file::File> {
		&self.document
	}
	pub fn viewports(&self) -> &std::collections::HashMap<u32, Viewport> {
		&self.viewports
	}
}

impl Default for Editor {
	fn default() -> Self {
		Self {
			document: None,
			viewports: std::collections::HashMap::new(),
		}
	}
}

#[derive(Debug)]
pub struct Viewport {
	id: u32,
	bounds: Rect<f32, f32>,
	event_queue: std::collections::VecDeque<EditorEvent>,
}

#[derive(Debug, PartialEq)]
pub struct PointerData {
	id: u32,
	contact: Rect<f32, f32>,
	pressure: f32,
	tilt: Vec2<f32>,
}

#[derive(Debug, PartialEq)]
pub enum EditorEvent {
	PointerDown(PointerData),
	PointerMove(PointerData),
	PointerUp(u8),
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[no_mangle]
pub extern "C" fn pxlr_editor_create() -> *mut Editor {
	Box::into_raw(Box::new(Editor::default()))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[no_mangle]
pub extern "C" fn pxlr_editor_destroy(handle: *mut Editor) {
	if !handle.is_null() {
		unsafe {
			Box::from_raw(handle);
		}
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[no_mangle]
pub extern "C" fn pxlr_editor_document_set(
	handle: *mut Editor,
	document_file_handle: *mut document_file::File,
) {
	if !handle.is_null() {
		let editor = unsafe { &mut *handle };
		if document_file_handle.is_null() {
			editor.document.take();
		} else {
			let file = unsafe { Box::from_raw(document_file_handle) };
			editor.document.replace(*file);
		}
	}
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[no_mangle]
pub extern "C" fn pxlr_editor_document_get(handle: *mut Editor) -> *const document_file::File {
	if !handle.is_null() {
		let editor = unsafe { &mut *handle };
		match editor.document() {
			None => std::ptr::null(),
			Some(document) => document as *const document_file::File,
		}
	} else {
		std::ptr::null()
	}
}
