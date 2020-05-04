use crate::patch::{IPatch, Patch, PatchMode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePatch {
	pub target: Uuid,
	pub name: String,
}

impl IPatch for RenamePatch {
	fn mode(&self) -> PatchMode {
		PatchMode::META
	}
}

pub trait Renamable<'a> {
	fn rename(&self, new_name: &'a str) -> Result<(Patch, Patch), RenameError>;
}

#[derive(Debug)]
pub enum RenameError {
	Unchanged,
}

impl std::fmt::Display for RenameError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			RenameError::Unchanged => write!(f, "Could not rename as name did not change."),
		}
	}
}

impl std::error::Error for RenameError {
	fn cause(&self) -> Option<&dyn std::error::Error> {
		None
	}
}
