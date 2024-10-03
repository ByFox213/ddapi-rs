#[cfg(test)]
mod tests {
    use reqwest::Client;
    use crate::{DDApi, MasterServer};
    use crate::{DDnetApi, DDstats};

    async fn setup() -> DDApi {
        let client = Client::new();
        DDApi::new(client)
    }

    #[tokio::test]
    async fn test_count_master() {
        let ddapi = setup().await;
        let result = ddapi.master(MasterServer::One).await;
        match result {
            Ok(res) => {
                println!("clients: {} from {}", res.count_clients(), &res.servers.len());
                assert_eq!(true, true);
            }
            Err(err) => {
                println!("{:?}", err);
                assert_eq!(false, true);
            }
        }
    }

    #[tokio::test]
    async fn test_get_clans_master() {
        let ddapi = setup().await;
        let result = ddapi.master(MasterServer::One).await;
        match result {
            Ok(res) => {
                let mut clans = res.get_clans(None);
                clans.truncate(5);

                println!("clans: {:?}", clans);
                assert_eq!(true, true);
            }
            Err(err) => {
                println!("{:?}", err);
                assert_eq!(false, true);
            }
        }
    }

    #[tokio::test]
    async fn test_players() {
        let ddapi = setup().await;
        let players = vec!["ByFox", "ban+eblan", "Gazebr"];

        for player in players {
            let result = ddapi.player(player).await;
            // println!("{:?}", result);
            assert_eq!(result.is_ok(), true);
        }
    }

    #[tokio::test]
    async fn test_master() {
        let ddapi = setup().await;
        let result = ddapi.master(MasterServer::One).await;
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