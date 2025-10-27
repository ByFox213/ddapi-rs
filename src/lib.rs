pub mod api;
pub mod error;
pub mod scheme;
mod tests;
pub mod util;

pub mod prelude {
    pub use crate::api::DDApi;
    pub use crate::util::*;

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
