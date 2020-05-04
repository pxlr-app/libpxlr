#![allow(dead_code)]
#![allow(unreachable_patterns)]

#[macro_use]
extern crate bitflags;

pub mod color;
mod document;
pub mod file;
mod group;
mod history;
mod node;
mod note;
pub mod parser;
pub mod patch;
pub mod sprite;
pub use self::document::*;
pub use self::group::*;
pub use self::history::*;
pub use self::node::*;
pub use self::note::*;
