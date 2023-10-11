// #![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]


#[cfg(feature = "core")]
pub use cobul_core::*;
#[cfg(feature = "props")]
pub use cobul_props::*;

#[cfg(feature = "simple")]
pub mod simple {
    pub use cobul_simple::*;
}

#[cfg(feature = "extra")]
pub mod extra {
    pub use cobul_extra::*;
}

#[cfg(feature = "icons")]
pub mod icons {
    pub use cobul_icons::{Brands, Icon, Regular, Solid};
}

#[cfg(feature = "form")]
pub use cobul_form::{Form, State, use_form, use_form_eq, use_form_with_model};


#[cfg(feature = "derive")]
pub use cobul_derive::Form;

