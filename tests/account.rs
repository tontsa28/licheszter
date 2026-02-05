use std::{error::Error, sync::LazyLock};

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
async fn account_profile() {
    // Run some test cases
    let result = LI.account_profile().await;
    assert!(
        result.is_ok(),
        "Failed to get profile information: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_profile().await;
    assert!(
        result.is_ok(),
        "Failed to get profile information: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.account_profile().await;
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
        "Failed to get account email: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_email().await;
    assert!(
        result.is_ok(),
        "Failed to get account email: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.account_email().await;
    assert!(result.is_err(), "Fetching account email did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn account_preferences() {
    // Run some test cases
    let result = LI.account_preferences().await;
    assert!(
        result.is_ok(),
        "Failed to get account preferences: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_preferences().await;
    assert!(
        result.is_ok(),
        "Failed to get account preferences: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.account_preferences().await;
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

    let result = DEFAULT.account_kid_mode().await;
    assert!(result.is_err(), "Checking account kid mode did not fail: {:?}", result.unwrap());
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

    let result = DEFAULT.account_kid_mode_set(true).await;
    assert!(result.is_err(), "Setting account kid mode did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn account_timeline() {
    // Run some test cases
    let result = LI.account_timeline(None, None).await;
    assert!(
        result.is_ok(),
        "Failed to get account timeline: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.account_timeline(Some(1704060000000), Some(10)).await;
    assert!(
        result.is_ok(),
        "Failed to get account timeline: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.account_timeline(Some(1704060000000), Some(30)).await;
    assert!(
        result.is_ok(),
        "Failed to get account timeline: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.account_timeline(None, None).await;
    assert!(result.is_err(), "Fetching account timeline did not fail: {:?}", result.unwrap());
}
