pub use base::{components::*, elements::*, form::*, layout::*, props::*};

#[cfg(feature = "extensions")]
pub use extensions::*;

#[cfg(feature = "forms")]
pub use forms::{use_form, Actions, UseFormHandle};

pub mod simple {
    #[cfg(feature = "simple")]
    pub use simple::*;
}

pub mod fa {
    #[cfg(feature = "fa")]
    pub use fa::{Brands, Icon, Regular, Solid};
}
