use crate::parser;
use async_std::io;
use async_std::io::prelude::*;
use math::blend::*;
use math::Lerp;
use nom::number::complete::{le_f32, le_u16, le_u8};
use nom::IResult;
use num_traits::identities::Zero;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::ops::{Add, Div, Mul, Sub};

pub trait Color: Copy {}

macro_rules! define_colors {
	{$(
		$idx:expr, $color:ident, ($($name:ident:$type:ty:$reader:ident),+);
	)+} => {

		#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
		pub enum ColorMode {
			$($color),+
		}

		impl parser::Parser for ColorMode {
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
			//      had to expand the macro manually. Keeping original
			//
			// async fn write(&self, storage: &mut S) -> io::Result<usize> {
			// 	let index: u16 = match self {
			// 		$(ColorMode::$color => $idx),+
			// 	};
			// 	storage.write(&index.to_le_bytes()).await?;
			// 	Ok(2)
			// }
			fn write<'life0, 'life1, 'async_trait, S>(
				&'life0 self,
				storage: &'life1 mut S,
			) -> ::core::pin::Pin<
				Box<dyn ::core::future::Future<Output = io::Result<usize>> + 'async_trait>,
			>
			where
				'life0: 'async_trait,
				'life1: 'async_trait,
				Self: 'async_trait,
				S: io::Read + io::Write + io::Seek + std::marker::Unpin,
			{
				async fn run<S>(
					_self: &ColorMode,
					storage: &mut S,
				) -> io::Result<usize>
				where
					S: io::Read + io::Write + io::Seek + std::marker::Unpin,
				{
					let index: u16 = match _self {
						$(ColorMode::$color => $idx),+
					};
					storage.write(&index.to_le_bytes()).await?;
					Ok(2)
				}
				Box::pin(run(self, storage))
			}
		}

		$(
			#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
			pub struct $color {
				$(pub $name: $type),+
			}

			impl $color {
				pub fn new($($name: $type),+) -> $color {
					$color { $($name: $name,)+ }
				}
			}

			impl Color for $color {}

			impl Default for $color {
				fn default() -> Self {
					$color { $($name: <$type as Zero>::zero(),)+ }
				}
			}

			impl Add for $color {
				type Output = $color;

				fn add(self, other: Self) -> Self {
					$color { $($name: ((self.$name as f64) + (other.$name as f64)) as $type,)+ }
				}
			}

			impl Sub for $color {
				type Output = $color;

				fn sub(self, other: Self) -> Self {
					$color { $($name: ((self.$name as f64) - (other.$name as f64)) as $type,)+ }
				}
			}

			impl Mul for $color {
				type Output = $color;

				fn mul(self, other: Self) -> Self {
					$color { $($name: ((self.$name as f64) * (other.$name as f64)) as $type,)+ }
				}
			}

			impl Div for $color {
				type Output = $color;

				fn div(self, other: Self) -> Self {
					$color { $($name: ((self.$name as f64) / (other.$name as f64)) as $type,)+ }
				}
			}

			impl Mul<$color> for f32 {
				type Output = $color;

				fn mul(self, other: $color) -> Self::Output {
					$color { $($name: (self * (other.$name as f32)) as $type,)+ }
				}
			}

			impl Mul<f32> for $color {
				type Output = $color;

				fn mul(self, other: f32) -> Self::Output {
					$color { $($name: ((self.$name as f32) * other) as $type,)+ }
				}
			}

			impl Blend for $color {
				type Output = $color;

				fn blend(from: &Self, to: &Self, mode: &BlendMode) -> Self {
					match mode {
						BlendMode::Normal => *to,
						BlendMode::Add => *from + *to,
						BlendMode::Subtract => *from - *to,
						BlendMode::Multiply => *from * *to,
						BlendMode::Divide => *from / *to,
						_ => *to,
					}
				}
			}

			impl Lerp<f32> for $color {
				type Output = $color;

				fn lerp_unclamped(from: Self, to: Self, factor: f32) -> Self::Output {
					from + (to - from) * factor
				}
			}

			impl parser::Parser for $color {
				fn parse(bytes: &[u8]) -> IResult<&[u8], $color> {
					$(
						let (bytes, $name) = $reader(bytes)?;
					)+
					Ok((bytes, $color { $($name),+ }))
				}

				// TODO Due to https://github.com/dtolnay/async-trait/issues/46
				//      had to expand the macro manually. Keeping original
				//
				// async fn write(&self, storage: &mut S) -> io::Result<usize> {
				// 	let mut b: usize = 0;
				// 	$(
				// 		b += storage.write(&self.$name.to_le_bytes()).await?;
				// 	)+
				// 	Ok(b)
				// }
				fn write<'life0, 'life1, 'async_trait, S>(
					&'life0 self,
					storage: &'life1 mut S,
				) -> ::core::pin::Pin<
					Box<dyn ::core::future::Future<Output = io::Result<usize>> + 'async_trait>,
				>
				where
					'life0: 'async_trait,
					'life1: 'async_trait,
					Self: 'async_trait,
					S: io::Read + io::Write + io::Seek + std::marker::Unpin,
				{
					async fn run<S>(
						_self: &$color,
						storage: &mut S,
					) -> io::Result<usize>
					where
						S: io::Read + io::Write + io::Seek + std::marker::Unpin,
					{
						let mut b: usize = 0;
						$(
							b += storage.write(&_self.$name.to_le_bytes()).await?;
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
	0, I, (i:u8:le_u8);
	1, IXYZ, (i:u8:le_u8, x:f32:le_f32, y:f32:le_f32, z:f32:le_f32);
	2, UV, (u:f32:le_f32, v:f32:le_f32);
	3, RGB, (r:u8:le_u8, g:u8:le_u8, b:u8:le_u8);
	4, RGBA, (r:u8:le_u8, g:u8:le_u8, b:u8:le_u8, a:u8:le_u8);
	5, RGBAXYZ, (r:u8:le_u8, g:u8:le_u8, b:u8:le_u8, a:u8:le_u8, x:f32:le_f32, y:f32:le_f32, z:f32:le_f32);
}
