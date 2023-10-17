

pub enum Margin {
    Left(Option<u32>),
    Right(Option<u32>),
    Top(Option<u32>),
    Bottom(Option<u32>),
    Horizontal(Option<u32>),
    Vertical(Option<u32>),
    All(Option<u32>)
}

impl Margin {
    pub fn left(value: u32) -> Self { Self::Left(Some(value)) }
}