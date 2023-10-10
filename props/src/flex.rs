use derive_more::Display;
use yew::Classes;
use yew::html::IntoPropValue;

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-flex-direction-{}")]
pub enum Direction {
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "row-reverse")]
    RowReverse,
    #[display(fmt = "column")]
    Column,
    #[display(fmt = "column-reverse")]
    ColumnReverse,
}

impl Into<Classes> for Direction {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-flex-{}")]
pub enum Wrap {
    #[display(fmt = "nowrap")]
    NoWrap,
    #[display(fmt = "wrap")]
    Wrap,
    #[display(fmt = "wrap-reverse")]
    WrapReverse,
}

impl Into<Classes> for Wrap {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-justify-content-{}")]
pub enum Justify {
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "space-between")]
    SpaceBetween,
    #[display(fmt = "space-around")]
    SpaceAround,
    #[display(fmt = "space-evenly")]
    SpaceEvenly,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "right")]
    Right,
}

impl Into<Classes> for Justify {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-align-content-{}")]
pub enum AlignContent {
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

impl Into<Classes> for AlignContent {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-align-items-{}")]
pub enum AlignItems {
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

impl Into<Classes> for AlignItems {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-align-self-{}")]
pub enum AlignSelf {
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

impl Into<Classes> for AlignSelf {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Default, Copy)]
pub struct Grow(pub Option<u32>);

impl Into<Classes> for Grow {
    fn into(self) -> Classes {
        match self.0 {
            Some(value) => Classes::from(format!("is-flex-grow-{}", value)),
            None => Classes::new(),
        }
    }
}

impl IntoPropValue<Grow> for u32 {
    fn into_prop_value(self) -> Grow {
        Grow(Some(self))
    }
}

#[derive(Clone, Debug, PartialEq, Default, Copy)]
pub struct Shrink(pub Option<u32>);

impl Into<Classes> for Shrink {
    fn into(self) -> Classes {
        match self.0 {
            Some(value) => Classes::from(format!("is-flex-grow-{}", value)),
            None => Classes::new(),
        }
    }
}

impl IntoPropValue<Shrink> for u32 {
    fn into_prop_value(self) -> Shrink {
        Shrink(Some(self))
    }
}
