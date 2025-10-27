pub mod api;
pub mod scheme;
mod tests;
pub mod util;
pub mod error;

pub mod prelude {
    pub use crate::util::*;
    pub use crate::api::DDApi;

    #[cfg(feature = "ddnet")]
    pub use crate::api::ddnet::DDnetApi;

    #[cfg(feature = "ddstats")]
    pub use crate::api::ddstats::DDstats;

    #[cfg(feature = "ddnet")]
    pub mod ddnet {
        pub use crate::api::ddnet::*;
        pub use crate::scheme::ddnet::*;
    }

    #[cfg(feature = "ddstats")]
    pub mod ddstats {
        pub use crate::api::ddstats::*;
        pub use crate::scheme::ddstats::*;
    }
}
