use derive_more::Display;
use yew::Classes;

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexDirection {
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "row-reverse")]
    RowReverse,
    #[display(fmt = "column")]
    Column,
    #[display(fmt = "column-reverse")]
    ColumnReverse,
}

impl Into<Classes> for FlexDirection {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexWrap {
    #[display(fmt = "wrap")]
    Wrap,
    #[display(fmt = "wrap-reverse")]
    WrapReverse,
    #[display(fmt = "nowrap")]
    NoWrap,
}

impl Into<Classes> for FlexWrap {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexJustify {
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "space-between")]
    Between,
    #[display(fmt = "space-around")]
    Around,
    #[display(fmt = "space-evenly")]
    Evenly,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
}

impl Into<Classes> for FlexJustify {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexAlignContent {
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "space-between")]
    Between,
    #[display(fmt = "space-around")]
    Around,
    #[display(fmt = "space-evenly")]
    Evenly,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "baseline")]
    Baseline,
}

impl Into<Classes> for FlexAlignContent {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexAlignItems {
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
}

impl Into<Classes> for FlexAlignItems {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexAlignSelf {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "stretch")]
    Stretch,
}

impl Into<Classes> for FlexAlignSelf {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum FlexSize {
    #[display(fmt = "flex-grow-0")]
    Grow0,
    #[display(fmt = "flex-grow-1")]
    Grow1,
    #[display(fmt = "flex-grow-2")]
    Grow2,
    #[display(fmt = "flex-grow-3")]
    Grow3,
    #[display(fmt = "flex-grow-4")]
    Grow4,
    #[display(fmt = "flex-grow-5")]
    Grow5,

    #[display(fmt = "flex-shrink-0")]
    Shrink0,
    #[display(fmt = "flex-shrink-1")]
    Shrink1,
    #[display(fmt = "flex-shrink-2")]
    Shrink2,
    #[display(fmt = "flex-shrink-3")]
    Shrink3,
    #[display(fmt = "flex-shrink-4")]
    Shrink4,
    #[display(fmt = "flex-shrink-5")]
    Shrink5,
}

impl Into<Classes> for FlexSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

