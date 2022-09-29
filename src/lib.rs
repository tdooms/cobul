pub use base::{components::*, elements::*, form::*, layout::*, model::*, props::*};

#[cfg(feature = "extensions")]
pub use extensions::*;

#[cfg(feature = "derive")]
pub use derive::Classable;

#[cfg(feature = "simple")]
pub mod simple {
    pub use simple::*;
}

#[cfg(feature = "fa")]
pub mod fa {
    pub use fa::{Brands, Icon, Regular, Solid};
}
