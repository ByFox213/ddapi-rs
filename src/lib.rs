mod api;
mod error;
mod scheme;
mod tests;
mod util;

pub mod prelude {
    pub use crate::api::DDApi;
    pub use crate::util::prelude::*;

    #[cfg(feature = "ddnet")]
    pub use crate::api::ddnet::DDnetApi;

    #[cfg(feature = "ddstats")]
    pub use crate::api::ddstats::DDstats;

    #[cfg(feature = "ddnet")]
    pub mod ddnet {
        pub use crate::api::ddnet::*;
        pub use crate::scheme::ddnet::prelude::*;
    }

    #[cfg(feature = "ddstats")]
    pub mod ddstats {
        pub use crate::api::ddstats::*;
        pub use crate::scheme::ddstats::*;
    }
}
