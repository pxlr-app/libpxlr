use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, OffscreenCanvas};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
	// This provides better error messages in debug mode.
	// It's disabled in release mode so it doesn't bloat up the file size.
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	// Your code goes here!
	console::log_1(&JsValue::from_str("Hello from Rust inside a worker!!!"));

	Ok(())
}

pub struct Editor {
	pub canvas: OffscreenCanvas,
	pub width: u32,
	pub height: u32,
}

#[wasm_bindgen]
impl Editor {
	pub fn load(canvas: OffscreenCanvas, width: u32, height: u32) -> *mut Editor {
		Box::into_raw(Box::new(Editor {
			canvas,
			width,
			height,
		}))
	}

	pub fn unload(editor: *mut Editor) {
		if !editor.is_null() {
			unsafe {
				Box::from_raw(editor);
			}
		}
	}

	pub fn resize(editor: *mut Editor, width: u32, height: u32) {
		unsafe {
			(*editor).width = width;
			(*editor).height = height;
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
enum Command {
	Ping,
}

#[wasm_bindgen]
pub fn process_command(editor: *mut Editor, input: JsValue) -> Result<(), JsValue> {
	if let Some(json) = input.as_string() {
		if let Ok(command) = serde_json::from_str(&json) {
			match command {
				Command::Ping => unsafe {
					console::log_1(&JsValue::from_str(&format!(
						"PONG {}x{}",
						(*editor).width,
						(*editor).height
					)));
				},
			}
		} else {
			console::log_2(&JsValue::from_str("unknown"), &JsValue::from_str(&json));
		}
	}

	Ok(())
}
