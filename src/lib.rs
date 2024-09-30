mod model;
mod error_ddapi;

use reqwest::{Client, Error};
use serde::de::DeserializeOwned;
use urlencoding::encode;
use error_ddapi::ApiError;
use model::*;

trait DDnetApi {
    async fn master(&self) -> Result<Master, ApiError>;
    async fn player(&self, player: &str) -> Result<DDPlayer, ApiError>;
    async fn query(&self, player: &str) -> Result<Query, ApiError>;
    async fn map(&self, map: &str) -> Result<DMap, ApiError>;
}

trait DDstats {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError>;
}

struct DDApi {
    client: Client,
}

impl DDApi {
    pub fn new(client: Client) -> DDApi {
        DDApi { client }
    }

    async fn send_request(&self, uri: &str) -> Result<String, Error> {
        self.client.get(uri).send().await?.text().await
    }

    async fn _generator<T>(&self, uri: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = self.send_request(uri).await?;
        let data: T = serde_json::from_str(&response)?;
        Ok(data)
    }

    fn encode_nickname(&self, nickname: &str) -> String {
        encode(nickname).as_ref().to_owned()
    }
}

impl DDnetApi for DDApi {
    async fn master(&self) -> Result<Master, ApiError> {
        self._generator("https://master1.ddnet.org/ddnet/15/servers.json").await
    }

    async fn player(&self, player: &str) -> Result<DDPlayer, ApiError> {
        self._generator(
            &*("https://ddnet.org/players/?json2=".to_owned() + &self.encode_nickname(player))
        ).await
    }
    async fn query(&self, player: &str) -> Result<Query, ApiError> {
        self._generator(
            &*("https://ddnet.org/players/?query=".to_owned() + &self.encode_nickname(player))
        ).await
    }
    async fn map(&self, map: &str) -> Result<DMap, ApiError> {
        self._generator(
            &*("https://ddnet.org/maps/?json=".to_owned() + &self.encode_nickname(map))
        ).await
    }
}


impl DDstats for DDApi {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(
            &*("https://ddstats.tw/player/json?player=".to_owned() + &self.encode_nickname(player))
        ).await
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> DDApi {
        let client = Client::new();
        DDApi::new(client)
    }

    #[tokio::test]
    async fn test_count_master() {
        let ddapi = setup().await;
        let result = ddapi.master().await;
        match result {
            Ok(res) => {
                println!("clients: {} from {}", res.count_client(), &res.servers.len());
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
        let result = ddapi.master().await;
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