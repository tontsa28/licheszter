use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::user::{Email, KidMode, Preferences, Timeline, User},
};

/// A struct for accessing the Account API endpoints.
pub struct AccountApi<'a> {
    pub(crate) client: &'a Licheszter,
}

impl AccountApi<'_> {
    /// Public information about the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn profile(&self) -> Result<User> {
        let url = self.client.req_url(UrlBase::Lichess, "api/account");
        let builder = self.client.client.get(url);

        self.client.to_model::<User>(builder).await
    }

    /// Read the email address of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn email(&self) -> Result<Email> {
        let url = self.client.req_url(UrlBase::Lichess, "api/account/email");
        let builder = self.client.client.get(url);

        self.client.to_model::<Email>(builder).await
    }

    /// Read the preferences of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn preferences(&self) -> Result<Preferences> {
        let url = self.client.req_url(UrlBase::Lichess, "api/account/preferences");
        let builder = self.client.client.get(url);

        self.client.to_model::<Preferences>(builder).await
    }

    /// Read the kid mode status of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn kid_mode(&self) -> Result<KidMode> {
        let url = self.client.req_url(UrlBase::Lichess, "api/account/kid");
        let builder = self.client.client.get(url);

        self.client.to_model::<KidMode>(builder).await
    }

    /// Set the kid mode status of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn kid_mode_set(&self, kid: bool) -> Result<()> {
        let url = self.client.req_url(UrlBase::Lichess, "api/account/kid");
        let builder = self.client.client.post(url).query(&[("v", kid)]);

        self.client.execute(builder).await
    }

    /// Get the timeline events of the logged in user.
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn timeline(&self, since: Option<u64>, amount: Option<u8>) -> Result<Timeline> {
        let url = self.client.req_url(UrlBase::Lichess, "api/timeline");
        let builder = self.client.client.get(url).query(&(("since", since), ("nb", amount)));

        self.client.to_model::<Timeline>(builder).await
    }
}
