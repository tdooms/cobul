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

/// Separator classes for breadcrumbs - [reference](https://bulma.io/documentation/components/breadcrumb/#alternative-separators)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "has-{}-separator")]
pub enum Separator {
    #[display(fmt = "arrow")]
    Arrow,
    #[display(fmt = "bullet")]
    Bullet,
    #[display(fmt = "dot")]
    Dot,
    #[display(fmt = "succeeds")]
    Succeeds,
}

impl Into<Classes> for Separator {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

/// Handle different column layouts for each breakpoint - [reference](https://bulma.io/documentation/columns/responsiveness/)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum Breakpoint {
    #[display(fmt = "mobile")]
    Mobile,
    #[display(fmt = "tablet")]
    Tablet,
    #[display(fmt = "desktop")]
    Desktop,
    #[display(fmt = "widescreen")]
    Widescreen,
    #[display(fmt = "fullhd")]
    FullHD,
}

/// Tile context modifiers - [reference](https://bulma.io/documentation/layout/tiles/#modifiers)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum TileCtx {
    #[display(fmt = "ancestor")]
    Ancestor,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "child")]
    Child,
}

impl Into<Classes> for TileCtx {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}
