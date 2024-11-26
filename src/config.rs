
#[derive(Copy, Clone, Debug)]
pub enum BlockSize {
    Tiny = 8,
    Small = 16,
    Medium = 32,
    Large = 64,
    Huge = 128,
    Giant = 256,
    Colossal = 512,
    Mammoth = 1024,
}

impl BlockSize {
    pub const MAX: usize = BlockSize::Mammoth as usize;

    pub fn categorize(size: usize) -> Option<Self> {
        match size {
            1..=8 => Some(BlockSize::Tiny),
            9..=16 => Some(BlockSize::Small),
            17..=32 => Some(BlockSize::Medium),
            33..=64 => Some(BlockSize::Large),
            65..=128 => Some(BlockSize::Huge),
            129..=256 => Some(BlockSize::Giant),
            257..=512 => Some(BlockSize::Colossal),
            513..=1024 => Some(BlockSize::Mammoth),
            _ => None,
        }
    }
}
