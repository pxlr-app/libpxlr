use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, OffscreenCanvas, WebGl2RenderingContext};

mod editor;
use editor::*;

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

#[derive(Debug, Default)]
pub struct Editor {
	pub canvas: Option<OffscreenCanvas>,
	pub width: u32,
	pub height: u32,
	pub viewports: Vec<Viewport>,
	pub gl: Option<WebGl2RenderingContext>,
}

#[wasm_bindgen]
impl Editor {
	pub fn init() -> *mut Editor {
		Box::into_raw(Box::new(Editor::default()))
	}

	pub fn uninit(editor: *mut Editor) {
		if !editor.is_null() {
			unsafe {
				Box::from_raw(editor);
			}
		}
	}

	pub fn send_command_with_canvas(editor: *mut Editor, cmd: JsValue, canvas: OffscreenCanvas) {
		if let Some(json) = cmd.as_string() {
			if let Ok(command) = serde_json::from_str::<Command>(&json) {
				let editor = unsafe { &mut *editor };
				match command {
					Command::Init => {
						let gl = canvas
							.get_context("webgl2")
							.unwrap()
							.unwrap()
							.dyn_into::<WebGl2RenderingContext>()
							.unwrap();

						gl.enable(WebGl2RenderingContext::SCISSOR_TEST);

						editor.canvas.replace(canvas);
						editor.gl.replace(gl);
					}
					_ => unreachable!(),
				}
			}
		}
	}

	pub fn send_command(editor: *mut Editor, cmd: JsValue) -> Result<(), JsValue> {
		if let Some(json) = cmd.as_string() {
			if let Ok(command) = serde_json::from_str::<Command>(&json) {
				let editor = unsafe { &mut *editor };
				match command {
					Command::Ping => {
						// console::log_1(&JsValue::from_str("PONG"));
					}
					Command::Resize { width, height } => {
						editor.width = width as u32;
						editor.height = height as u32;
						if let Some(canvas) = &editor.canvas {
							canvas.set_width(editor.width);
							canvas.set_height(editor.height);
						}
					}
					Command::AddViewport { viewport } => {
						editor.viewports.push(viewport);
					}
					Command::RemoveViewport { key } => {
						editor.viewports = editor
							.viewports
							.drain(..)
							.filter(|viewport| viewport.key != key)
							.collect();
					}
					Command::UpdateViewport { viewport } => {
						if let Some(old_viewport) = editor
							.viewports
							.iter_mut()
							.find(|vp| vp.key == viewport.key)
						{
							*old_viewport = viewport;
						}
					}
					Command::Draw => {
						if let Some(gl) = editor.gl.as_ref() {
							// let (width, height) = (editor.width, editor.height);

							// console::log_1(&JsValue::from_str(&format!(
							// 	"DRAW {}x{}",
							// 	width, height
							// )));
							// for viewport in &editor.viewports {
							// 	console::log_1(&JsValue::from_str(&format!(
							// 		"VIEWPORT {} {},{},{},{}",
							// 		viewport.key,
							// 		viewport.bounds.top,
							// 		viewport.bounds.right,
							// 		viewport.bounds.bottom,
							// 		viewport.bounds.left
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
							for viewport in &editor.viewports {
								let w = (viewport.bounds.right - viewport.bounds.left) as i32;
								let h = (viewport.bounds.bottom - viewport.bounds.top) as i32;
								let x = viewport.bounds.left as i32;
								let y = editor.height as i32 - (viewport.bounds.top as i32 + h);
								// console::log_1(&JsValue::from_str(&format!(
								// 	"DRAW {} : {},{},{},{}",
								// 	viewport.key, x, y, w, h
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
						}
					}
					_ => panic!("Unknown command"),
				}
			} else {
				console::log_2(&JsValue::from_str("unknown"), &JsValue::from_str(&json));
			}
		}

		Ok(())
	}
}
