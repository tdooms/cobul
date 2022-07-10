pub use base::{components::*, elements::*, form::*, layout::*, props::*};
#[cfg(feature = "fa")]
pub use fa::Icons;
#[cfg(feature = "forms")]
pub use forms::{use_form, Actions, UseFormHandle};
#[cfg(feature = "slider")]
pub use slider::Slider;
#[cfg(feature = "switch")]
pub use switch::Switch;

pub mod simple {
    #[cfg(feature = "simple")]
    pub use simple::*;
}

pub mod custom {
    #[cfg(feature = "custom")]
    pub use custom::*;
}
