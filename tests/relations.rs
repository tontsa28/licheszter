use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::client::Licheszter;

// Connect to test accounts
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build()
});

#[tokio::test]
async fn relations_followed_users_list() {
    // Run some test cases
    let mut result = LI.relations_followed_users_list().await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to fetch followers: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = BOT0.relations_followed_users_list().await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to fetch followers: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = Licheszter::new().relations_followed_users_list().await;
    assert!(result.is_err(), "Fetching followers did not fail");
}
