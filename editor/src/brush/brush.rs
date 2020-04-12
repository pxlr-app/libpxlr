use document::sprite::Stencil;

pub trait Brush<T>
where
    T: Default + Copy,
{
    fn paint_on_stencil(&self, stencil: &Stencil<T>);
}
