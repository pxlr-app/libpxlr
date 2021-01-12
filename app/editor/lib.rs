use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, OffscreenCanvas, WebGl2RenderingContext};

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

pub struct Viewport {
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
	pub left: f32,
}

pub struct Editor {
	pub canvas: OffscreenCanvas,
	pub width: u32,
	pub height: u32,
	pub viewports: HashMap<String, Viewport>,
	pub gl: WebGl2RenderingContext,
}

#[wasm_bindgen]
impl Editor {
	pub fn init(canvas: OffscreenCanvas, width: u32, height: u32) -> *mut Editor {
		let gl = canvas
			.get_context("webgl2")
			.unwrap()
			.unwrap()
			.dyn_into::<WebGl2RenderingContext>()
			.unwrap();

		gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

		Box::into_raw(Box::new(Editor {
			canvas,
			width,
			height,
			viewports: HashMap::new(),
			gl,
		}))
	}

	pub fn uninit(editor: *mut Editor) {
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

	pub fn add_viewport(
		editor: *mut Editor,
		id: String,
		top: f32,
		right: f32,
		bottom: f32,
		left: f32,
	) {
		unsafe {
			(*editor).viewports.insert(
				id,
				Viewport {
					top,
					right,
					bottom,
					left,
				},
			);
		}
	}

	pub fn remove_viewport(editor: *mut Editor, id: String) {
		unsafe {
			(*editor).viewports.remove(&id);
		}
	}

	pub fn update_viewport(
		editor: *mut Editor,
		id: String,
		top: f32,
		right: f32,
		bottom: f32,
		left: f32,
	) {
		unsafe {
			if let Some(viewport) = (*editor).viewports.get_mut(&id) {
				viewport.top = top;
				viewport.right = right;
				viewport.bottom = bottom;
				viewport.left = left;
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd")]
enum Command {
	Ping,
	Draw,
}

#[wasm_bindgen]
pub fn process_command(editor: *mut Editor, input: JsValue) -> Result<(), JsValue> {
	if let Some(json) = input.as_string() {
		if let Ok(command) = serde_json::from_str(&json) {
			match command {
				Command::Ping => {
					console::log_1(&JsValue::from_str("PONG"));
				}
				Command::Draw => unsafe {
					let editor = &*editor;

					let (gl, width, height) = (&editor.gl, editor.width, editor.height);

					console::log_1(&JsValue::from_str(&format!("DRAW {}x{}", width, height)));
					// for (id, viewport) in &(*editor).viewports {
					// 	console::log_1(&JsValue::from_str(&format!(
					// 		"VIEWPORT {} {},{},{},{}",
					// 		id, viewport.top, viewport.right, viewport.bottom, viewport.left
					// 	)));
					// }

					gl.clear_color(0.0, 0.0, 0.0, 0.0);
					gl.clear(
						WebGl2RenderingContext::COLOR_BUFFER_BIT
							| WebGl2RenderingContext::DEPTH_BUFFER_BIT,
					);

					let colors = vec![
						(0.9568f32, 0.5529f32, 0.8667f32, 1.0f32),
						(0.8117f32, 0.5725f32, 0.6745f32, 1.0f32),
						(0.4078f32, 0.4862f32, 0.9647f32, 1.0f32),
						(0.2784f32, 0.9058f32, 0.9137f32, 1.0f32),
						(0.6078f32, 0.9725f32, 0.6431f32, 1.0f32),
					];

					let mut i = 2usize;
					for (_, viewport) in &editor.viewports {
						let x = ((viewport.left / 100f32) * width as f32) as i32;
						let y = (((100f32 - viewport.bottom) / 100f32) * height as f32) as i32;
						let w = (((viewport.right - viewport.left) / 100f32) * width as f32) as i32;
						let h =
							(((viewport.bottom - viewport.top) / 100f32) * height as f32) as i32;
						// console::log_1(&JsValue::from_str(&format!(
						// 	"DRAW {} : {},{},{},{}",
						// 	id, x, y, w, h
						// )));
						gl.viewport(x, y, w, h);
						gl.scissor(x, y, w, h);
						let (r, g, b, a) = colors[i % 5];
						gl.clear_color(r, g, b, a);
						gl.clear_depth(1f32);
						gl.clear(
							WebGl2RenderingContext::COLOR_BUFFER_BIT
								| WebGl2RenderingContext::DEPTH_BUFFER_BIT,
						);

						i += 1usize;
					}
				},
			}
		} else {
			console::log_2(&JsValue::from_str("unknown"), &JsValue::from_str(&json));
		}
	}

	Ok(())
}
