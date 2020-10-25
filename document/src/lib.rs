#![feature(const_btree_new)]

#[macro_use]
extern crate enum_display_derive;

pub use document_derive::*;
pub mod any;
pub mod canvas;
pub mod color;
pub mod command;
pub mod file;
pub mod node;
pub mod parser;

pub mod prelude {
	pub use super::{any::*, canvas::*, color::*, command::*, file::*, node::*, parser};
	pub use document_derive::*;
	pub use math::{Extent2, Mat3, Rect, Vec2};
	pub use serde::{Deserialize, Serialize};
	pub use std::io;
	#[cfg(not(feature = "arc"))]
	pub use std::rc::{Rc as Arc, Weak};
	#[cfg(feature = "arc")]
	pub use std::sync::{Arc, Weak};
	pub use uuid::Uuid;
}
