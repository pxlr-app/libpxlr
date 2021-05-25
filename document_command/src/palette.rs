use crate::{Command, CommandType};
use color::Rgba;
use document_core::{HasColors, Node, NodeType};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum PaletteError {
	ExistingColor(Rgba),
	InvalidColor(Rgba),
}

impl std::fmt::Display for PaletteError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			PaletteError::ExistingColor(color) => write!(f, "Color {} already present", color),
			PaletteError::InvalidColor(color) => write!(f, "Invalid color {}", color),
		}
	}
}

impl std::error::Error for PaletteError {}

pub trait ColorManager: HasColors + Node {
	fn add_color(&self, color: Rgba) -> Result<CommandType, PaletteError> {
		let color_found = self.colors().iter().find(|c| **c == color).is_some();
		if color_found {
			Err(PaletteError::ExistingColor(color))
		} else {
			Ok(CommandType::AddPaletteColor(AddPaletteColorCommand {
				target: *self.id(),
				color: color,
			}))
		}
	}
	fn remove_color(&self, color: Rgba) -> Result<CommandType, PaletteError> {
		let old_position = self.colors().iter().position(|c| *c == color);
		match old_position {
			Some(_) => Ok(CommandType::RemovePaletteColor(RemovePaletteColorCommand {
				target: *self.id(),
				color,
			})),
			None => Err(PaletteError::InvalidColor(color)),
		}
	}
	fn move_color(&self, color: Rgba, position: usize) -> Result<CommandType, PaletteError> {
		let old_position = self.colors().iter().position(|c| *c == color);
		match old_position {
			Some(_) => Ok(CommandType::MovePaletteColor(MovePaletteColorCommand {
				target: *self.id(),
				color,
				position,
			})),
			None => Err(PaletteError::InvalidColor(color)),
		}
	}
}

impl<N: HasColors + Node> ColorManager for N {}

#[derive(Debug, Clone)]
pub struct AddPaletteColorCommand {
	pub target: Uuid,
	pub color: Rgba,
}

#[derive(Debug, Clone)]
pub struct RemovePaletteColorCommand {
	pub target: Uuid,
	pub color: Rgba,
}

#[derive(Debug, Clone)]
pub struct MovePaletteColorCommand {
	pub target: Uuid,
	pub color: Rgba,
	pub position: usize,
}

impl Command for AddPaletteColorCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Palette(node) => {
				let mut cloned = node.clone();
				let mut colors: Vec<Rgba> = cloned.colors().iter().map(|c| c.clone()).collect();
				colors.push(self.color.clone());
				cloned.set_colors(colors);
				Some(NodeType::Palette(cloned))
			}
			_ => None,
		}
	}
}

impl Command for RemovePaletteColorCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Palette(node) => {
				let mut cloned = node.clone();
				let colors: Vec<Rgba> = cloned
					.colors()
					.iter()
					.filter_map(|color| {
						if *color == self.color {
							None
						} else {
							Some(color.clone())
						}
					})
					.collect();
				cloned.set_colors(colors);
				Some(NodeType::Palette(cloned))
			}
			_ => None,
		}
	}
}

impl Command for MovePaletteColorCommand {
	fn target(&self) -> &Uuid {
		&self.target
	}
	fn execute_impl(&self, node: &NodeType) -> Option<NodeType> {
		match node {
			NodeType::Palette(node) => {
				let mut cloned = node.clone();
				let mut colors: Vec<Rgba> = cloned.colors().iter().map(|c| c.clone()).collect();
				let child = colors.remove(self.position);
				if self.position > colors.len() {
					colors.push(child);
				} else {
					colors.insert(self.position, child);
				}
				cloned.set_colors(colors);
				Some(NodeType::Palette(cloned))
			}
			_ => None,
		}
	}
}
