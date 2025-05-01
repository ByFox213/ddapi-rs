#[cfg(test)]
mod tests {
    use crate::api::ddnet::DDnetApi;
    #[cfg(feature = "ddstats")]
    use crate::api::ddstats::DDstats;
    use crate::api::DDApi;

    const PLAYERS: [&str; 3] = ["ByFox", "ban+eblan", "Gazebr"];
    const PLAYER: &str = "ByFox";
    const MAP: &str = "Multeasymap";

    // ddnet
    #[tokio::test]
    async fn test_players() {
        let ddapi = DDApi::new();

        for player in &PLAYERS {
            let result = ddapi.player(player).await;
            assert_eq!(result.is_ok(), true);
        }
    }

    #[tokio::test]
    async fn test_master() {
        let ddapi = DDApi::new();
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_query() {
        let ddapi = DDApi::new();
        let result = ddapi.query(PLAYER).await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_map() {
        let ddapi = DDApi::new();
        let result = ddapi.map(MAP).await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_releases_map() {
        let ddapi = DDApi::new();
        let result = ddapi.releases_map().await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_status() {
        let ddapi = DDApi::new();
        let result = ddapi.status().await;
        assert_eq!(result.is_ok(), true)
    }

    // ddstats
    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_player() {
        let ddapi = DDApi::new();
        for player in &PLAYERS {
            let result = ddapi.s_player(player).await;
            assert_eq!(result.is_ok(), true);
        }
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_maps() {
        let ddapi = DDApi::new();
        let result = ddapi.s_maps().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_profile() {
        let ddapi = DDApi::new();
        let result = ddapi.s_profile(PLAYER).await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    #[cfg(feature = "ddstats")]
    async fn test_s_map() {
        let ddapi = DDApi::new();
        let result = ddapi.s_map(MAP).await;
        assert_eq!(result.is_ok(), true);
    }
}
