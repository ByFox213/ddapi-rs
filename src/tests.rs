#[cfg(test)]
mod tests {
    use crate::{DDApi, DDnetApi, DDstats};
    use reqwest::Client;

    async fn setup() -> DDApi {
        let client = Client::new();
        DDApi::new(client)
    }

    #[tokio::test]
    async fn test_count_master() {
        let ddapi = setup().await;
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_get_clans_master() {
        let ddapi = setup().await;
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true);
    }

    #[tokio::test]
    async fn test_players() {
        let ddapi = setup().await;
        let players = vec!["ByFox", "ban+eblan", "Gazebr"];

        for player in players {
            let result = ddapi.player(player).await;
            assert_eq!(result.is_ok(), true);
        }
    }

    #[tokio::test]
    async fn test_master() {
        let ddapi = setup().await;
        let result = ddapi.master().await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_query() {
        let ddapi = setup().await;
        let result = ddapi.query("ByFox").await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_map() {
        let ddapi = setup().await;
        let result = ddapi.map("Multeasymap").await;
        assert_eq!(result.is_ok(), true)
    }

    #[tokio::test]
    async fn test_stats_player() {
        let ddapi = setup().await;
        let result = ddapi.splayer("ByFox").await;
        assert_eq!(result.is_ok(), true)
    }
}
