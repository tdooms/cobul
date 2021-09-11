use derive_more::Display;
use yew::Classes;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum Alignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

impl Into<Classes> for Alignment {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

/// Addons alignment classes.
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "has-addons-{}")]
pub enum AddonsAlignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

impl Default for AddonsAlignment {
    fn default() -> Self {
        AddonsAlignment::Left
    }
}

impl Into<Classes> for AddonsAlignment {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}
