#![cfg(feature = "relations")]

use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::client::Licheszter;

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .unwrap()
        .build()
});

static DEFAULT: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .build()
});

#[tokio::test]
async fn relations_followed_users_list() {
    // Run some test cases
    let mut stream = LI.relations().followed_users_list().await.unwrap();
    while let Some(result) = stream.next().await {
        assert!(
            result.is_ok(),
            "Failed to get followers: {:?}",
            result.unwrap_err().source().unwrap()
        );
    }

    let mut stream = BOT0.relations().followed_users_list().await.unwrap();
    while let Some(result) = stream.next().await {
        assert!(
            result.is_ok(),
            "Failed to get followers: {:?}",
            result.unwrap_err().source().unwrap()
        );
    }

    let result = DEFAULT.relations().followed_users_list().await;
    assert!(result.is_err(), "Fetching followers did not fail");
}

#[tokio::test]
async fn relations_follow() {
    // Run some test cases
    let result = LI.relations().follow("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to follow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations().follow("Li").await;
    assert!(
        result.is_ok(),
        "Failed to follow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations().follow("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Following non-existent player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn relations_unfollow() {
    // Run some test cases
    let result = LI.relations().unfollow("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to unfollow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations().unfollow("Li").await;
    assert!(
        result.is_ok(),
        "Failed to unfollow a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations().unfollow("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Unfollowing non-existent player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn relations_block() {
    // Run some test cases
    let result = LI.relations().block("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to block a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations().block("Li").await;
    assert!(
        result.is_ok(),
        "Failed to block a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations().block("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Blocking non-existent player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn relations_unblock() {
    // Run some test cases
    let result = LI.relations().unblock("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to unblock a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.relations().unblock("Li").await;
    assert!(
        result.is_ok(),
        "Failed to unblock a player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.relations().unblock("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Unblocking non-existent player did not fail: {:?}",
        result.unwrap()
    );
}
