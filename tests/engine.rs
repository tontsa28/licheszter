#![cfg(feature = "engine")]

use std::{error::Error, sync::LazyLock};

use licheszter::{
    client::Licheszter, config::engine::ExternalEngineOptions, models::engine::UciVariant,
};

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
    // Create an external engine for testing
    let options = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken")
        .provider_data("arbitrarydata")
        .variants(&[UciVariant::Chess]);
    LI.external_engine().create(&options).await.unwrap();

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

#[tokio::test]
async fn external_engine_create() {
    // Create options for testing
    let options1 = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken");
    let options2 = ExternalEngineOptions::new(64, 1, "Perch", "afullysecuresecrettoken")
        .provider_data("arbitrarydata")
        .variants(&[UciVariant::Chess]);
    let options3 = ExternalEngineOptions::new(0, 0, "", "");

    // Run some tests
    let result = LI.external_engine().create(&options1).await;
    assert!(
        result.is_ok(),
        "Failed to create external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.external_engine().create(&options2).await;
    assert!(
        result.is_ok(),
        "Failed to create external engine: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.external_engine().create(&options3).await;
    assert!(
        result.is_err(),
        "Creating external engine did not fail: {:?}",
        result.unwrap()
    );

    let result = DEFAULT.external_engine().create(&options1).await;
    assert!(
        result.is_err(),
        "Creating external engine did not fail: {:?}",
        result.unwrap()
    );
}
