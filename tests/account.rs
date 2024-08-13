use std::{error::Error, sync::LazyLock};

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
async fn account_profile() {
    // Run some test cases
    let result = LI.account_profile().await;
    assert!(
        result.is_ok(),
        "Failed to fetch profile information: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_profile().await;
    assert!(
        result.is_ok(),
        "Failed to fetch profile information: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().account_profile().await;
    assert!(
        result.is_err(),
        "Fetching profile information did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn account_email() {
    // Run some test cases
    let result = LI.account_email().await;
    assert!(
        result.is_ok(),
        "Failed to fetch account email: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_email().await;
    assert!(
        result.is_ok(),
        "Failed to fetch account email: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().account_email().await;
    assert!(
        result.is_err(),
        "Fetching account email did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn account_preferences() {
    // Run some test cases
    let result = LI.account_preferences().await;
    assert!(
        result.is_ok(),
        "Failed to fetch account preferences: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_preferences().await;
    assert!(
        result.is_ok(),
        "Failed to fetch account preferences: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().account_preferences().await;
    assert!(
        result.is_err(),
        "Fetching account preferences did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn account_kid_mode() {
    // Run some test cases
    let result = LI.account_kid_mode().await;
    assert!(
        result.is_ok(),
        "Failed to check account kid mode: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_kid_mode().await;
    assert!(
        result.is_ok(),
        "Failed to check account kid mode: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().account_kid_mode().await;
    assert!(
        result.is_err(),
        "Checking account kid mode did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn account_kid_mode_set() {
    // Run some test cases
    let result = LI.account_kid_mode_set(false).await;
    assert!(
        result.is_ok(),
        "Failed to set account kid mode: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_kid_mode_set(false).await;
    assert!(
        result.is_ok(),
        "Failed to set account kid mode: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().account_kid_mode_set(true).await;
    assert!(
        result.is_err(),
        "Setting account kid mode did not fail: {:?}",
        result.unwrap()
    );
}