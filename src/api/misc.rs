use std::pin::Pin;

use crate::{
    client::{Licheszter, UrlBase},
    error::{Error, ErrorKind, Result, StringError},
    models::{board::Event, user::BasicUser},
};
use futures_util::Stream;
use reqwest::header::{self, HeaderMap, HeaderValue};

impl Licheszter {
    /// Stream the events reaching a Lichess user in real time.
    /// When the stream opens, all current challenges and games are sent.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn connect(&self) -> Result<Pin<Box<dyn Stream<Item = Result<Event>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/stream/event");
        let builder = self.client.get(url);

        self.to_stream::<Event>(builder).await
    }

    /// Get online bots.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response stream cannot be created.
    pub async fn bots_online(
        &self,
        bots: u8,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<BasicUser>> + Send>>> {
        let url = self.req_url(UrlBase::Lichess, "api/bot/online");
        let builder = self.client.get(url).query(&[("nb", bots)]);

        self.to_stream::<BasicUser>(builder).await
    }

    /// Upgrade a Lichess player account into a bot account.
    /// This method only works for bot accounts.
    /// The account MUST NOT have any games played before upgrading.
    /// This action is irreversible.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The API request fails or the response cannot be deserialized
    /// - The authentication token contains invalid characters (non-visible ASCII, newlines, etc.)
    pub async fn bot_account_upgrade(&self, token: &str) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, "api/bot/account/upgrade");

        // Securely construct the authorization header
        let bearer = format!("Bearer {token}");
        let mut auth_header = HeaderValue::from_str(&bearer).map_err(|e| {
            Error::new(
                ErrorKind::InvalidAuthToken,
                StringError(format!("Authentication token contains invalid characters: {}", e)),
            )
        })?;
        auth_header.set_sensitive(true);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, auth_header);

        let builder = self.client.post(url).headers(headers);

        self.execute(builder).await
    }
}
