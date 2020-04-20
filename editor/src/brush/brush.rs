use document::sprite::Stencil;

pub trait Brush {
	fn get_stencil(&self) -> Stencil;
}
