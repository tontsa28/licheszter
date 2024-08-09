use futures_util::Stream;

use crate::{client::Licheszter, error::Result, models::user::User};

impl Licheszter {
    /// Get a list of users followed by the logged in user.
    pub async fn relations_followed_users_list(&self) -> Result<impl Stream<Item = Result<User>>> {
        let mut url = self.base_url();
        url.set_path("api/rel/following");
        let builder = self.client.get(url);

        self.to_model_stream::<User>(builder).await
    }
}
