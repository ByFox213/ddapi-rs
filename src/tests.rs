#[cfg(test)]
mod tests {
    use crate::api::ddnet::DDnetApi;
    use crate::api::ddstats::DDstats;
    use crate::DDApi;

    #[tokio::test]
    async fn test_count_master() {
        let ddapi = DDApi::new();
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_get_clans_master() {
        let ddapi = DDApi::new();
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_players() {
        let ddapi = DDApi::new();
        let players = vec!["ByFox", "ban+eblan", "Gazebr"];

        for player in players {
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
        let result = ddapi.query("ByFox").await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_map() {
        let ddapi = DDApi::new();
        let result = ddapi.map("Multeasymap").await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_stats_player() {
        let ddapi = DDApi::new();
        let result = ddapi.splayer("ByFox").await;
        assert_eq!(result.is_ok(), true)
    }
}
