use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{
        common::OkResponse,
        user::{Email, KidMode, Preferences, Timeline, User},
    },
};

impl Licheszter {
    /// Public information about the logged in user.
    pub async fn account_profile(&self) -> Result<User> {
        let url = self.req_url(UrlBase::Lichess, "api/account");
        let builder = self.client.get(url);

        self.into::<User>(builder).await
    }

    /// Read the email address of the logged in user.
    pub async fn account_email(&self) -> Result<Email> {
        let url = self.req_url(UrlBase::Lichess, "api/account/email");
        let builder = self.client.get(url);

        self.into::<Email>(builder).await
    }

    /// Read the preferences of the logged in user.
    pub async fn account_preferences(&self) -> Result<Preferences> {
        let url = self.req_url(UrlBase::Lichess, "api/account/preferences");
        let builder = self.client.get(url);

        self.into::<Preferences>(builder).await
    }

    /// Read the kid mode status of the logged in user.
    pub async fn account_kid_mode(&self) -> Result<KidMode> {
        let url = self.req_url(UrlBase::Lichess, "api/account/kid");
        let builder = self.client.get(url);

        self.into::<KidMode>(builder).await
    }

    /// Set the kid mode status of the logged in user.
    pub async fn account_kid_mode_set(&self, kid: bool) -> Result<()> {
        let url = self.req_url(UrlBase::Lichess, "api/account/kid");
        let builder = self.client.post(url).query(&[("v", kid)]);

        self.into::<OkResponse>(builder).await?;
        Ok(())
    }

    /// Get the timeline events of the logged in user.
    pub async fn account_timeline(&self, since: Option<u64>, amount: Option<u8>) -> Result<Timeline> {
        let url = self.req_url(UrlBase::Lichess, "api/timeline");
        let builder = self.client.get(url).query(&(("since", since), ("nb", amount)));

        self.into::<Timeline>(builder).await
    }
}
