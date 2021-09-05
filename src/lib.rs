pub use cobul_base;

#[cfg(feature = "custom")]
pub use cobul_custom;

#[cfg(feature = "fa")]
pub use cobul_fa;

pub mod prelude {
    pub use cobul_base::prelude::*;

    #[cfg(feature = "custom")]
    pub use cobul_custom::prelude::*;

    #[cfg(feature = "fa")]
    pub use cobul_fa::Icons;
}
