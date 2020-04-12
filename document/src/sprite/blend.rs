#[derive(Copy, Clone, Debug)]
pub enum BlendMode {
    Normal,
    Multiply,
    Divide,
    Add,
    Subtract,
    Difference,
    Screen,
    Darken,
    Lighten,
}

pub trait Blend {
    type Output;

    fn blend(from: &Self, to: &Self, mode: &BlendMode) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Blend for u32 {
        type Output = u32;
        fn blend(from: &u32, to: &u32, mode: &BlendMode) -> u32 {
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
    fn it_blends() {
        assert_eq!(Blend::blend(&128u32, &32u32, &BlendMode::Normal), 32u32);
        assert_eq!(Blend::blend(&128u32, &32u32, &BlendMode::Add), 160u32);
        assert_eq!(Blend::blend(&128u32, &32u32, &BlendMode::Subtract), 96u32);
        assert_eq!(Blend::blend(&128u32, &32u32, &BlendMode::Multiply), 4096u32);
        assert_eq!(Blend::blend(&128u32, &32u32, &BlendMode::Divide), 4u32);
    }
}
