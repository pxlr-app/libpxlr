use math::{Extent2, Vec2};
use std::any::Any;

use uuid::Uuid;

use crate::node::Node;
use crate::patch::*;
use crate::sprite::Interpolation;

pub trait Layer: Node {
    fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>);
    fn resize(
        &self,
        size: Extent2<u32>,
        interpolation: Interpolation,
    ) -> (ResizePatch, Box<dyn PatchImpl>);
}

pub trait LayerImpl: Layer {
    fn as_any(&self) -> &dyn Any;
}

impl<T> LayerImpl for T
where
    T: Layer + Any,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CropPatch {
    pub target: Uuid,
    pub offset: Vec2<u32>,
    pub size: Extent2<u32>,
}

impl Patch for CropPatch {
    fn target(&self) -> Uuid {
        self.target
    }
}

pub struct ResizePatch {
    pub target: Uuid,
    pub size: Extent2<u32>,
    pub interpolation: Interpolation,
}

impl Patch for ResizePatch {
    fn target(&self) -> Uuid {
        self.target
    }
}
