use crate::{
    client::Licheszter,
    error::Result,
    models::{
        common::OkResponse,
        user::{Email, KidMode, Preferences, Timeline, User},
    },
};

impl Licheszter {
    /// Public information about the logged in user.
    pub async fn account_profile(&self) -> Result<User> {
        let mut url = self.base_url();
        url.set_path("api/account");
        let builder = self.client.get(url);

        self.to_model::<User>(builder).await
    }

    /// Read the email address of the logged in user.
    pub async fn account_email(&self) -> Result<Email> {
        let mut url = self.base_url();
        url.set_path("api/account/email");
        let builder = self.client.get(url);

        self.to_model::<Email>(builder).await
    }

    /// Read the preferences of the logged in user.
    pub async fn account_preferences(&self) -> Result<Preferences> {
        let mut url = self.base_url();
        url.set_path("api/account/preferences");
        let builder = self.client.get(url);

        self.to_model::<Preferences>(builder).await
    }

    /// Read the kid mode status of the logged in user.
    pub async fn account_kid_mode(&self) -> Result<KidMode> {
        let mut url = self.base_url();
        url.set_path("api/account/kid");
        let builder = self.client.get(url);

        self.to_model::<KidMode>(builder).await
    }

    /// Set the kid mode status of the logged in user.
    pub async fn account_kid_mode_set(&self, kid: bool) -> Result<OkResponse> {
        let mut url = self.base_url();
        url.set_path("api/account/kid");
        let builder = self.client.post(url).query(&[("v", kid)]);

        self.to_model::<OkResponse>(builder).await
    }

    /// Get the timeline events of the logged in user.
    pub async fn account_timeline(&self, since: Option<u64>, nb: Option<u8>) -> Result<Timeline> {
        let mut url = self.base_url();
        url.set_path("api/timeline");
        let builder = self.client.get(url).query(&(("since", since), ("nb", nb)));

        self.to_model::<Timeline>(builder).await
    }
}
