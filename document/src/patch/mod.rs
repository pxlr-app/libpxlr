use crate::{
	any::{Any, Downcast},
	node::Node,
};
use std::fmt::Debug;
use uuid::Uuid;

mod node;
pub use node::*;

pub trait Patch: Any + Debug {
	fn target(&self) -> Uuid;
}
impl Downcast for dyn Patch {}

pub trait Patchable {
	fn patch(&self, patch: &dyn Patch) -> Option<Box<dyn Node>>;
}

pub type PatchRegistry<'b> = Vec<&'static str>;

static mut PATCHES: Option<PatchRegistry> = None;

pub trait Registry {
	fn registry<'b>() -> &'static Option<PatchRegistry<'b>>;
	fn init_registry<'b>(map: PatchRegistry<'b>);
}

impl Registry for dyn Patch {
	fn registry<'b>() -> &'static Option<PatchRegistry<'b>> {
		unsafe {
			std::mem::transmute::<&Option<PatchRegistry<'static>>, &Option<PatchRegistry<'b>>>(
				&PATCHES,
			)
		}
	}
	fn init_registry<'b>(map: PatchRegistry<'b>) {
		unsafe {
			PATCHES = Some(std::mem::transmute::<
				PatchRegistry<'b>,
				PatchRegistry<'static>,
			>(map));
		}
	}
}
