use crate::patch::{IPatch, Patch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFoldPatch {
	pub target: Uuid,
	pub folded: bool,
}

impl IPatch for SetFoldPatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}

pub trait IFoldable {
	fn set_fold(&self, folded: bool) -> Result<(Patch, Patch), SetFoldError>;
}

#[derive(Debug)]
pub enum SetFoldError {
	Unchanged,
}

impl std::fmt::Display for SetFoldError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			SetFoldError::Unchanged => write!(f, "Could not set folded as value did not change."),
		}
	}
}

impl std::error::Error for SetFoldError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}
