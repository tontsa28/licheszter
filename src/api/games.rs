use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::game::{UserGame, UserGames},
};

impl Licheszter {
    /// Get the ongoing games of the current user.
    /// The most urgent games are listed first.
    pub async fn games_ongoing(&self, games: u8) -> Result<Vec<UserGame>> {
        let url = self.req_url(UrlBase::Lichess, "api/account/playing");
        let builder = self.client.get(url).query(&[("nb", games)]);

        Ok(self.into::<UserGames>(builder).await?.now_playing)
    }
}
