use math::Extent2;

pub struct Stencil<T>
where
    T: Default + Copy,
{
    pub size: Extent2<u32>,
    pub mask: Vec<u8>,
    pub data: Vec<T>,
}

pub struct StencilDataIterator<'a, T> {
    bit_offset: u32,
    data_offset: u32,
    width: u32,
    height: u32,
    mask: &'a [u8],
    data: &'a [T],
}

impl<'a, T> Iterator for StencilDataIterator<'a, T> {
    type Item = (u32, u32, &'a T);

    fn next(&mut self) -> Option<(u32, u32, &'a T)> {
        let bit_count = (self.mask.len() * 8) as u32;
        while self.bit_offset < bit_count {
            let bit_offset = self.bit_offset;
            self.bit_offset += 1;
            let uchar_offset = (bit_offset / 8) | 0;
            let bit = self.mask[uchar_offset as usize] & (1 << (bit_offset - uchar_offset * 8));
            if bit != 0 {
                let x = bit_offset % self.width as u32;
                let y = (bit_offset / self.width as u32) | 0;
                self.data_offset += 1;
                return Some((x, y, &self.data[(self.data_offset - 1) as usize]));
            }
        }
        return None;
    }
}

impl<T> Stencil<T>
where
    T: Default + Copy,
{
    pub fn from_buffer(size: Extent2<u32>, buffer: &[T]) -> Stencil<T> {
        assert_eq!((size.w * size.h) as usize, buffer.len());
        let closest_bytes = 1 + (((size.w * size.h) - 1) / 8); // ceil
        let mut mask: Vec<u8> = vec![255u8; closest_bytes as usize];
        for i in buffer.len()..mask.len() * 8 {
            let p = (i / 8) | 0;
            let m = 1 << (i - p * 8);
            mask[p] ^= m;
        }

        let data: Vec<T> = buffer.to_vec();
        Stencil::<T> {
            size: size,
            mask: mask,
            data: data,
        }
    }

    pub fn iter(&self) -> StencilDataIterator<T> {
        StencilDataIterator {
            bit_offset: 0,
            data_offset: 0,
            width: self.size.w,
            height: self.size.h,
            mask: &self.mask[..],
            data: &self.data[..],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_from_buffer() {
        let s = Stencil::from_buffer(Extent2::new(2, 2), &[1u8, 2u8, 3u8, 4u8]);
        assert_eq!(*s.mask, [15u8]); // layout ⠛
        assert_eq!(*s.data, [1u8, 2u8, 3u8, 4u8]);
    }

    #[test]
    fn it_iter() {
        let s = Stencil {
            size: Extent2::new(2, 2),
            mask: vec![15u8], // layout ⠛
            data: vec![1u8, 2u8, 3u8, 4u8],
        };
        let mut i = s.iter();
        assert_eq!(i.next(), Some((0, 0, &1u8)));
        assert_eq!(i.next(), Some((1, 0, &2u8)));
        assert_eq!(i.next(), Some((0, 1, &3u8)));
        assert_eq!(i.next(), Some((1, 1, &4u8)));
        assert_eq!(i.next(), None);

        let s = Stencil {
            size: Extent2::new(2, 2),
            mask: vec![9u8], // layout ⠑
            data: vec![1u8, 4u8],
        };
        let mut i = s.iter();
        assert_eq!(i.next(), Some((0, 0, &1u8)));
        assert_eq!(i.next(), Some((1, 1, &4u8)));
        assert_eq!(i.next(), None);
    }
}
