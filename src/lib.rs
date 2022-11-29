#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Test
//!
//!

#[cfg(feature = "base")]
pub use cobul_base::{components::*, elements::*, form::*, layout::*, model::*, props::*};

#[cfg(feature = "extensions")]
pub use cobul_extensions::*;

#[cfg(feature = "simple")]
pub mod simple {
    pub use cobul_simple::*;
}

#[cfg(feature = "fa")]
pub mod fa {
    pub use cobul_fa::{Brands, Icon, Regular, Solid};
}
