// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]


#[cfg(feature = "core")]
pub use cobul_core::*;

#[cfg(feature = "props")]
pub use cobul_props::*;

#[cfg(feature = "form")]
pub mod form {
    pub use cobul_form::*;
}

#[cfg(feature = "extra")]
pub mod extra {
    pub use cobul_extra::*;
}

#[cfg(feature = "icons")]
pub mod icons {
    pub use cobul_icons::{Brands, Icon, Regular, Solid};
}
