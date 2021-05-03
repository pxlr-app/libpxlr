use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sampling {
	Nearest,
	Bilinear,
	// Bicubic,
}
