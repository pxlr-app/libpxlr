use async_std::io::{Result, Seek, SeekFrom, Write};
use async_std::task::{Context, Poll};
use std::pin::Pin;
pub struct Void;

impl Write for Void {
	fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, _buf: &[u8]) -> Poll<Result<usize>> {
		Poll::Ready(Ok(0))
	}
	fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
		Poll::Ready(Ok(()))
	}
	fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
		Poll::Ready(Ok(()))
	}
}

impl Write for &Void {
	fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, _buf: &[u8]) -> Poll<Result<usize>> {
		Poll::Ready(Ok(0))
	}
	fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
		Poll::Ready(Ok(()))
	}
	fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
		Poll::Ready(Ok(()))
	}
}

impl Seek for Void {
	fn poll_seek(self: Pin<&mut Self>, _cx: &mut Context<'_>, _pos: SeekFrom) -> Poll<Result<u64>> {
		Poll::Ready(Ok(0))
	}
}

impl Seek for &Void {
	fn poll_seek(self: Pin<&mut Self>, _cx: &mut Context<'_>, _pos: SeekFrom) -> Poll<Result<u64>> {
		Poll::Ready(Ok(0))
	}
}
