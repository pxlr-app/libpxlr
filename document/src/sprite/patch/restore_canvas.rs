use math::Extent2;
use uuid::Uuid;

use crate::patch::Patch;

pub struct RestoreCanvasPatch<T>
where
    T: Default + Copy,
{
    pub target: Uuid,
    pub name: String,
    pub size: Extent2<u32>,
    pub data: Vec<T>,
}

impl<T> Patch for RestoreCanvasPatch<T>
where
    T: Default + Copy,
{
    fn target(&self) -> Uuid {
        self.target
    }
}
