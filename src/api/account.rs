use crate::{client::Licheszter, error::Result, models::user::User};

impl Licheszter {
    /// Public information about the logged in user.
    pub async fn account_profile(&self) -> Result<User> {
        let mut url = self.base_url();
        url.set_path("api/account");
        let builder = self.client.get(url);

        self.to_model::<User>(builder).await
    }
}