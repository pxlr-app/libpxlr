mod group;
mod canvas;
mod interpolation;
mod layer;
mod patch;
mod stencil;

pub use self::group::*;
pub use self::canvas::*;
pub use self::interpolation::*;
pub use self::layer::*;
pub use self::patch::*;
pub use self::stencil::*;

pub type Sprite = Group;