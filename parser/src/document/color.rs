use crate::parser;
use document::color::*;
use futures::io;
use nom::number::complete::{le_f32, le_u16, le_u8};
use nom::IResult;

macro_rules! define_colors {
	{$(
		$idx:expr, $color:ident, ($($name:ident:$type:ty:$reader:ident),+);
	)+} => {

		impl parser::IParser for ColorMode {
			fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
				let (bytes, index) = le_u16(bytes)?;
				let value = match index {
					$(
						$idx => ColorMode::$color,
					)+
					_ => panic!("Unknown chunk type"),
				};
				Ok((bytes, value))
			}

			// TODO Due to https://github.com/dtolnay/async-trait/issues/46
			//		had to expand the macro manually
			//
			// fn write<S>(&self, storage: &mut S) -> io::Result<usize> where S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin {
			// 	let index: u16 = match self {
			// 		$(ColorMode::$color => $idx),+
			// 	};
			// 	storage.write_all(&index.to_le_bytes())?;
			// 	Ok(2)
			// }
			fn write<'a, 'b, 'async_trait, S>(
				&'a self,
				storage: &'b mut S,
			) -> ::core::pin::Pin<
				Box<dyn ::core::future::Future<Output = io::Result<usize>> + std::marker::Send + 'async_trait>
			>
			where
				'a: 'async_trait,
				'b: 'async_trait,
				Self: std::marker::Sync + 'async_trait,
				S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
			{
				async fn run<S>(
					color_mode: &ColorMode,
					storage: &mut S,
				) -> io::Result<usize>
				where
				S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
				{
					let index: u16 = match color_mode {
						$(ColorMode::$color => $idx),+
					};
					storage.write_all(&index.to_le_bytes()).await?;
					Ok(2)
				}
				Box::pin(run(self, storage))
			}
		}

		$(
			impl parser::IParser for $color {
				fn parse(bytes: &[u8]) -> IResult<&[u8], $color> {
					$(
						let (bytes, $name) = $reader(bytes)?;
					)+
					Ok((bytes, $color { $($name),+ }))
				}

				// TODO Due to https://github.com/dtolnay/async-trait/issues/46
				//		had to expand the macro manually
				//
				// fn write<S>(&self, storage: &mut S) -> io::Result<usize> where S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin {
				// 	let mut b: usize = 0;
				// 	$(
				// 		b += storage.write_all(&self.$name.to_le_bytes())?;
				// 	)+
				// 	Ok(b)
				// }
				fn write<'a, 'b, 'async_trait, S>(
					&'a self,
					storage: &'b mut S,
				) -> ::core::pin::Pin<
					Box<dyn ::core::future::Future<Output = io::Result<usize>> + std::marker::Send + 'async_trait>,
				>
				where
					'a: 'async_trait,
					'b: 'async_trait,
					Self: std::marker::Sync + 'async_trait,
					S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
				{
					async fn run<S>(
						color: &$color,
						storage: &mut S,
					) -> io::Result<usize>
					where
						S: io::AsyncWriteExt + std::marker::Send + std::marker::Unpin,
					{
						let mut b: usize = 0;
						$(
							let buf = &color.$name.to_le_bytes();
							storage.write_all(buf).await?;
							b += buf.len();
						)+
						Ok(b)
					}
					Box::pin(run(self, storage))
				}
			}
		)+
	}
}

define_colors! {
	0, Grey, (i:u8:le_u8);
	1, RGBA, (r:u8:le_u8, g:u8:le_u8, b:u8:le_u8, a:u8:le_u8);
	2, UV, (u:f32:le_f32, v:f32:le_f32, a:u8:le_u8);
	3, Normal, (x:f32:le_f32, y:f32:le_f32, z:f32:le_f32);
}
