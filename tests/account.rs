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