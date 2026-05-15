#![cfg(feature = "engine")]

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
async fn external_engine_list() {
    // Run some test cases
    let result = LI.external_engine().list().await;
    assert!(
        result.is_ok(),
        "Failed to list external engines: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().list().await;
    assert!(
        result.is_ok(),
        "Failed to list external engines: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.external_engine().list().await;
    assert!(
        result.is_err(),
        "Listing external engines did not fail: {:?}",
        result.unwrap()
    );
}
