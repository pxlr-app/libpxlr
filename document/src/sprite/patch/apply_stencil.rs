use math::Vec2;
use uuid::Uuid;

use crate::patch::Patch;
use crate::sprite::{BlendMode, Stencil};

pub struct ApplyStencilPatch<T>
where
    T: Default + Copy,
{
    pub target: Uuid,
    pub stencil: Stencil<T>,
    pub offset: Vec2<u32>,
    pub blend_mode: BlendMode,
}

impl<T> Patch for ApplyStencilPatch<T>
where
    T: Default + Copy,
{
    fn target(&self) -> Uuid {
        self.target
    }
}
