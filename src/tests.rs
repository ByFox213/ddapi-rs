#[cfg(test)]
mod tests {
    use crate::prelude::*;

    const PLAYER: &str = "ByFox";
    const PLAYER_MAPPER: &str = "Gazebr";
    const MAP: &str = "Multeasymap";

    // ddnet
    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_players() {
        let ddapi = DDApi::new();

        let result = ddapi.player(&PLAYER).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_master() {
        let ddapi = DDApi::new();
        let result = ddapi.master().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_query() {
        let ddapi = DDApi::new();
        let result = ddapi.query(PLAYER).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_query_map() {
        let ddapi = DDApi::new();
        let result = ddapi.query_map(PLAYER).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_query_mapper() {
        let ddapi = DDApi::new();
        let result = ddapi.query_mapper(PLAYER_MAPPER).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_map() {
        let ddapi = DDApi::new();
        let result = ddapi.map(MAP).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_releases_map() {
        let ddapi = DDApi::new();
        let result = ddapi.releases_map().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_status() {
        let ddapi = DDApi::new();
        let result = ddapi.status().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_skins() {
        let ddapi = DDApi::new();
        let result = ddapi.skins().await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    #[cfg(feature = "ddnet")]
    async fn test_latest_finishes() {
        let ddapi = DDApi::new();
        let result = ddapi.latest_finish().await;
        assert!(result.is_ok())
    }

    // ddstats
    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_player() {
        let ddapi = DDApi::new();
        let result = ddapi.s_player(&PLAYER).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_maps() {
        let ddapi = DDApi::new();
        let result = ddapi.s_maps().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_profile() {
        let ddapi = DDApi::new();
        let result = ddapi.s_profile(PLAYER).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_map() {
        let ddapi = DDApi::new();
        let result = ddapi.s_map(MAP).await;
        assert!(result.is_ok());
    }
}
