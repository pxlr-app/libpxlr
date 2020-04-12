use document::sprite::{Blend, Interpolation, Stencil};
use math::{Lerp, Vec2};

use crate::brush::Brush;

struct Line<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    from: Vec2<u32>,
    to: Vec2<u32>,
    width: u32,
    color: T,
    interpolation: Interpolation,
}

impl<T> Brush<T> for Line<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    fn paint_on_stencil(&self, stencil: &Stencil<T>) {}
}
