use async_std::{
	io::{Read, Result, Seek, SeekFrom, Write},
	task::{Context, Poll},
};
use js_sys::Uint8Array;
use std::{future::Future, pin::Pin};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, FileSystemWritableFileStream, WriteCommandType, WriteParams};

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

impl Seek for FileReader {
	fn poll_seek(self: Pin<&mut Self>, _cx: &mut Context<'_>, pos: SeekFrom) -> Poll<Result<u64>> {
		// Translate position to relative offset
		let self_mut = self.get_mut();
		let new_offset = match pos {
			SeekFrom::Current(pos) => self_mut.offset + pos,
			SeekFrom::End(pos) => pos as i64,
			SeekFrom::Start(pos) => pos as i64,
		};
		self_mut.offset = new_offset;
		Poll::Ready(Ok(new_offset as u64))
	}
}

impl Read for FileReader {
	fn poll_read(
		self: Pin<&mut Self>,
		cx: &mut async_std::task::Context<'_>,
		buf: &mut [u8],
	) -> Poll<Result<usize>> {
		let len = buf.len();
		let mut_self = self.get_mut();
		let future = mut_self.future.take();
		match future {
			None => {
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
				let mut future = JsFuture::from(blob.array_buffer());
				let pinned = Pin::new(&mut future);
				// Poll future to initiate waker
				let _ = Future::poll(pinned, cx);
				mut_self.future.replace(future);
				Poll::Pending
			}
			Some(mut js_future) => {
				let pinned = Pin::new(&mut js_future);
				match Future::poll(pinned, cx) {
					Poll::Pending => {
						mut_self.future.replace(js_future);
						Poll::Pending
					}
					Poll::Ready(res) => {
						let buffer = js_sys::Uint8Array::new(&res.unwrap());
						buffer.copy_to(buf);
						mut_self.offset += len as i64;
						Poll::Ready(Ok(len))
					}
				}
			}
		}
	}
}

pub struct FileWriter {
	stream: FileSystemWritableFileStream,
	offset: i64,
	size: u64,
	future: Option<JsFuture>,
}

impl FileWriter {
	pub fn new(stream: FileSystemWritableFileStream, size: u64) -> Self {
		Self {
			stream,
			offset: 0,
			size,
			future: None,
		}
	}

	pub async fn close(&mut self) {
		let _ = JsFuture::from(self.stream.close()).await;
		self.future = None;
	}
}

impl Seek for FileWriter {
	fn poll_seek(self: Pin<&mut Self>, _cx: &mut Context<'_>, pos: SeekFrom) -> Poll<Result<u64>> {
		// Translate position to relative offset
		let self_mut = self.get_mut();
		let new_offset = match pos {
			SeekFrom::Current(pos) => self_mut.offset + pos,
			SeekFrom::End(pos) => self_mut.size as i64 + pos as i64,
			SeekFrom::Start(pos) => pos as i64,
		};
		self_mut.offset = new_offset;
		Poll::Ready(Ok(new_offset as u64))
	}
}

impl Write for FileWriter {
	fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
		let len = buf.len();
		let mut_self = self.get_mut();
		let future = mut_self.future.take();
		match future {
			None => {
				let mut params = WriteParams::new(WriteCommandType::Write);
				let data: Uint8Array = buf.into();
				let data: JsValue = data.into();
				params.data(Some(&data));
				params.position(Some(mut_self.offset as f64));
				let mut future = JsFuture::from(mut_self.stream.write_with_write_params(&params));
				let pinned = Pin::new(&mut future);
				// Poll future to initiate waker
				let _ = Future::poll(pinned, cx);
				mut_self.future.replace(future);
				Poll::Pending
			}
			Some(mut js_future) => {
				let pinned = Pin::new(&mut js_future);
				match Future::poll(pinned, cx) {
					Poll::Pending => {
						mut_self.future.replace(js_future);
						Poll::Pending
					}
					Poll::Ready(_) => {
						mut_self.offset += len as i64;
						Poll::Ready(Ok(len))
					}
				}
			}
		}
	}

	fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
		Poll::Ready(Ok(()))
	}

	fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
		let mut_self = self.get_mut();
		let future = mut_self.future.take();
		match future {
			None => {
				let mut future = JsFuture::from(mut_self.stream.close());
				let pinned = Pin::new(&mut future);
				// Poll future to initiate waker
				let _ = Future::poll(pinned, cx);
				mut_self.future.replace(future);
				Poll::Pending
			}
			Some(mut js_future) => {
				let pinned = Pin::new(&mut js_future);
				match Future::poll(pinned, cx) {
					Poll::Pending => {
						mut_self.future.replace(js_future);
						Poll::Pending
					}
					Poll::Ready(_) => Poll::Ready(Ok(())),
				}
			}
		}
	}
}
