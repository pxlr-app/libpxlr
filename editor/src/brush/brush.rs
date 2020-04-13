use document::sprite::Stencil;

pub trait Brush<T>
where
	T: Default + Copy,
{
	fn get_stencil(&self) -> Stencil<T>;
}
