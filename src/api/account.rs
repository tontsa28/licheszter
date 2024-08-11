use crate::{client::Licheszter, error::Result, models::{common::OkResponse, user::{Email, KidMode, User}}};

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
}