mod latest_finishes;
mod map;
mod master;
mod player;
mod query;
mod query_map;
mod query_mapper;
mod releases_maps;
mod skins;
mod status;

pub mod prelude {
    pub use crate::scheme::ddnet::latest_finishes::*;
    pub use crate::scheme::ddnet::map::*;
    pub use crate::scheme::ddnet::master::*;
    pub use crate::scheme::ddnet::player::*;
    pub use crate::scheme::ddnet::query::*;
    pub use crate::scheme::ddnet::query_map::*;
    pub use crate::scheme::ddnet::query_mapper::*;
    pub use crate::scheme::ddnet::releases_maps::*;
    pub use crate::scheme::ddnet::skins::*;
    pub use crate::scheme::ddnet::status::*;
}
