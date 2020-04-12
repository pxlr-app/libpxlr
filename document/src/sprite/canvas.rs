use math::{Extent2, Lerp, Mat2, Vec2};
use std::iter::FromIterator;
use std::rc::Rc;

use uuid::Uuid;

use crate::node::*;
use crate::patch::*;
use crate::sprite::*;

pub struct Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    pub id: Uuid,
    pub name: Rc<String>,
    pub size: Rc<Extent2<u32>>,
    pub data: Rc<Vec<T>>,
}

impl<T> Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    pub fn new(id: Option<Uuid>, name: &str, size: Extent2<u32>, data: Vec<T>) -> Canvas<T> {
        Canvas::<T> {
            id: id.or(Some(Uuid::new_v4())).unwrap(),
            name: Rc::new(name.to_owned()),
            size: Rc::new(size),
            data: Rc::new(data),
        }
    }

    fn apply_stencil(
        &self,
        offset: Vec2<u32>,
        blend_mode: BlendMode,
        stencil: Stencil<T>,
    ) -> (ApplyStencilPatch<T>, RestoreCanvasPatch<T>) {
        assert_eq!(stencil.size.w + offset.x <= self.size.w, true);
        assert_eq!(stencil.size.h + offset.y <= self.size.h, true);
        (
            ApplyStencilPatch::<T> {
                target: self.id,
                offset: offset,
                blend_mode: blend_mode,
                stencil: stencil,
            },
            RestoreCanvasPatch::<T> {
                target: self.id,
                name: (*self.name).to_owned(),
                size: (*self.size).clone(),
                data: (*self.data).to_owned(),
            },
        )
    }
}

impl<T> std::ops::Index<(u32, u32)> for Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    type Output = T;

    fn index(&self, (x, y): (u32, u32)) -> &T {
        let i = (x * self.size.h + y) as usize;
        &self.data[i]
    }
}

impl<T> Node for Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    fn id(&self) -> Uuid {
        self.id
    }
}

impl<T> Layer for Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T> + 'static,
{
    fn crop(&self, offset: Vec2<u32>, size: Extent2<u32>) -> (CropPatch, Box<dyn PatchImpl>) {
        assert_eq!(size.w + offset.x <= self.size.w, true);
        assert_eq!(size.h + offset.y <= self.size.h, true);
        (
            CropPatch {
                target: self.id,
                offset: offset,
                size: size,
            },
            Box::new(RestoreCanvasPatch::<T> {
                target: self.id,
                name: (*self.name).to_owned(),
                size: (*self.size).clone(),
                data: (*self.data).to_owned(),
            }),
        )
    }

    fn resize(
        &self,
        size: Extent2<u32>,
        interpolation: Interpolation,
    ) -> (ResizePatch, Box<dyn PatchImpl>) {
        (
            ResizePatch {
                target: self.id,
                size: size,
                interpolation: interpolation,
            },
            Box::new(RestoreCanvasPatch::<T> {
                target: self.id,
                name: (*self.name).to_owned(),
                size: (*self.size).clone(),
                data: (*self.data).to_owned(),
            }),
        )
    }
}

impl<'a, T> Renamable<'a> for Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T>,
{
    fn rename(&self, new_name: &'a str) -> (RenamePatch, RenamePatch) {
        (
            RenamePatch {
                target: self.id,
                name: new_name.to_owned(),
            },
            RenamePatch {
                target: self.id,
                name: (*self.name).to_owned(),
            },
        )
    }
}

impl<T> Patchable for Canvas<T>
where
    T: Default + Copy + Blend<Output = T> + Lerp<f32, Output = T> + Blend<Output = T> + 'static,
{
    fn patch(&self, patch: &dyn PatchImpl) -> Option<Box<Self>> {
        if patch.target() == self.id {
            if let Some(patch) = patch.as_any().downcast_ref::<RenamePatch>() {
                return Some(Box::new(Canvas::<T> {
                    id: self.id,
                    name: Rc::new(patch.name.clone()),
                    size: self.size.clone(),
                    data: self.data.clone(),
                }));
            } else if let Some(patch) = patch.as_any().downcast_ref::<RestoreCanvasPatch<T>>() {
                return Some(Box::new(Canvas::<T> {
                    id: self.id,
                    name: Rc::new(patch.name.to_owned()),
                    size: Rc::new(patch.size),
                    data: Rc::new(patch.data.to_owned()),
                }));
            } else if let Some(patch) = patch.as_any().downcast_ref::<CropPatch>() {
                assert_eq!(patch.size.w + patch.offset.x <= self.size.w, true);
                assert_eq!(patch.size.h + patch.offset.y <= self.size.h, true);
                let mut data: Vec<T> =
                    vec![Default::default(); (patch.size.w * patch.size.h) as usize];
                for i in 0..data.len() {
                    let x = patch.offset.x + ((i as u32) % patch.size.w);
                    let y = patch.offset.y + ((i as u32) / patch.size.w);
                    data[i] = self[(x, y)];
                }
                return Some(Box::new(Canvas::<T> {
                    id: self.id,
                    name: self.name.clone(),
                    size: Rc::new(patch.size),
                    data: Rc::new(data),
                }));
            } else if let Some(patch) = patch.as_any().downcast_ref::<ResizePatch>() {
                let mut data: Vec<T> =
                    vec![Default::default(); (patch.size.w * patch.size.h) as usize];
                patch.interpolation.interpolate(
                    &self.size,
                    &self.data,
                    &patch.size,
                    &mut data,
                    Mat2::scaling_2d(Vec2::new(
                        ((self.size.w - 1) as f32) / (patch.size.w as f32),
                        ((self.size.h - 1) as f32) / (patch.size.h as f32),
                    )),
                );
                return Some(Box::new(Canvas::<T> {
                    id: self.id,
                    name: self.name.clone(),
                    size: Rc::new(patch.size),
                    data: Rc::new(data),
                }));
            } else if let Some(patch) = patch.as_any().downcast_ref::<ApplyStencilPatch<T>>() {
                let mut data: Vec<T> = Vec::from_iter(self.data.iter().cloned());
                for (x, y, d) in patch.stencil.iter() {
                    let x = x + patch.offset.x;
                    let y = y + patch.offset.y;
                    let i = (x * self.size.h + y) as usize;
                    data[i] = Blend::blend(&self.data[i], &d, &patch.blend_mode);
                }
                return Some(Box::new(Canvas::<T> {
                    id: self.id,
                    name: self.name.clone(),
                    size: self.size.clone(),
                    data: Rc::new(data),
                }));
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use math::{Extent2, Vec2};

    impl Blend for u8 {
        type Output = u8;

        fn blend(from: &u8, to: &u8, mode: &BlendMode) -> u8 {
            match mode {
                BlendMode::Normal => *to,
                BlendMode::Add => from + to,
                BlendMode::Subtract => from - to,
                BlendMode::Multiply => from * to,
                BlendMode::Divide => from / to,
                _ => *to,
            }
        }
    }

    #[test]
    fn from_buffer() {
        let c1 = Canvas::new(
            None,
            "canvas",
            Extent2::new(2u32, 2u32),
            vec![255u8, 128u8, 64u8, 32u8],
        );

        assert_eq!(*c1.data, vec![255, 128, 64, 32]);
    }

    #[test]
    fn it_crops() {
        let c1 = Canvas::new(
            None,
            "canvas",
            Extent2::new(2u32, 2u32),
            vec![255u8, 128u8, 64u8, 32u8],
        );

        let (patch, _) = c1.crop(Vec2::new(1, 0), Extent2::new(1, 2));
        let c2 = c1.patch(&patch).unwrap();

        assert_eq!(*c2.data, vec![64, 32]);
    }

    #[test]
    fn it_resizes() {
        let c1 = Canvas::new(
            None,
            "canvas",
            Extent2::new(2u32, 2u32),
            vec![255u8, 128u8, 64u8, 32u8],
        );

        let (patch, _) = c1.resize(Extent2::new(4, 4), Interpolation::Nearest);
        let c2 = c1.patch(&patch).unwrap();

        assert_eq!(
            *c2.data,
            vec![255, 255, 128, 128, 255, 255, 128, 128, 64, 64, 32, 32, 64, 64, 32, 32]
        );

        let (patch, _) = c1.resize(Extent2::new(4, 4), Interpolation::Bilinear);
        let c2 = c1.patch(&patch).unwrap();
        assert_eq!(
            *c2.data,
            vec![255, 223, 192, 160, 207, 181, 156, 130, 160, 140, 120, 100, 112, 98, 84, 70]
        );

        let (patch, _) = c1.resize(Extent2::new(2, 1), Interpolation::Nearest);
        let c2 = c1.patch(&patch).unwrap();

        assert_eq!(*c2.data, vec![255, 64]);
    }

    #[test]
    fn it_apply_patch() {
        let c1 = Canvas::new(
            None,
            "canvas",
            Extent2::new(2u32, 2u32),
            vec![196u8, 128u8, 64u8, 32u8],
        );

        let (patch, _) = c1.apply_stencil(
            Vec2::new(0, 0),
            BlendMode::Normal,
            Stencil::from_buffer(Extent2::new(2, 2), &[255u8, 255u8, 255u8, 255u8]),
        );
        let c2 = c1.patch(&patch).unwrap();
        assert_eq!(*c2.data, vec![255, 255, 255, 255]);

        let (patch, _) = c1.apply_stencil(
            Vec2::new(0, 0),
            BlendMode::Normal,
            Stencil {
                size: Extent2::new(2, 2),
                mask: vec![9u8], // layout ⠑
                data: vec![255u8, 255u8],
            },
        );
        let c2 = c1.patch(&patch).unwrap();
        assert_eq!(*c2.data, vec![255, 128, 64, 255]);
    }
}
