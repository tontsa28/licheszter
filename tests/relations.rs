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

#[tokio::test]
async fn relations_follow() {
    // Run some test cases
    let result = LI.relations_follow("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to follow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations_follow("Li").await;
    assert!(
        result.is_ok(),
        "Failed to follow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations_follow("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Following non-existent player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn relations_unfollow() {
    // Run some test cases
    let result = LI.relations_unfollow("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to unfollow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations_unfollow("Li").await;
    assert!(
        result.is_ok(),
        "Failed to unfollow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations_unfollow("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Unfollowing non-existent player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn relations_block() {
    // Run some test cases
    let result = LI.relations_block("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to block a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations_block("Li").await;
    assert!(
        result.is_ok(),
        "Failed to block a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations_block("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Blocking non-existent player did not fail: {:?}",
        result.unwrap()
    );
}
