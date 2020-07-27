#![feature(const_btree_new)]

pub use document_derive::*;
pub mod any;
pub mod color;
pub mod file;
pub mod node;
pub mod parser;
pub mod patch;
pub mod stencil;

pub mod prelude {
	pub use super::{
		any::*, color::*, file::*, node::*, parser, patch, patch::Patchable, stencil::*,
	};
	pub use document_derive::*;
	pub use math::{Extent2, Vec2};
	pub use serde::{Deserialize, Serialize};
	pub use std::io;
	#[cfg(not(feature = "arc"))]
	pub use std::rc::{Rc as Arc, Weak};
	#[cfg(feature = "arc")]
	pub use std::sync::{Arc, Weak};
	pub use uuid::Uuid;
}
