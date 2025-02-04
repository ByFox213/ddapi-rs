use crate::DDApi;
use crate::error_ddapi::ApiError;
use crate::scheme::ddstats::Player;

#[allow(dead_code)]
pub trait DDstats {
    fn splayer(
        &self,
        player: &str,
    ) -> impl std::future::Future<Output = Result<Player, ApiError>> + Send;
}

impl DDstats for DDApi {
    async fn splayer(&self, player: &str) -> Result<Player, ApiError> {
        self._generator(&*format!(
            "https://ddstats.tw/player/json?player={}",
            self.encode_nickname(player).await
        ))
            .await
    }
}