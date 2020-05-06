use crate::patch::{IPatch, Patch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetVisibilityPatch {
	pub target: Uuid,
	pub visibility: bool,
}

impl IPatch for SetVisibilityPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}

pub trait IVisible {
	fn set_visibility(&self, visible: bool) -> Result<(Patch, Patch), SetVisibilityError>;
}

#[derive(Debug)]
pub enum SetVisibilityError {
	Unchanged,
}

impl std::fmt::Display for SetVisibilityError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			SetVisibilityError::Unchanged => {
				write!(f, "Could not set visiblity as value did not change.")
			}
		}
	}
}

impl std::error::Error for SetVisibilityError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}