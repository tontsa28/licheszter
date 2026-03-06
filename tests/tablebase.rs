#![cfg(feature = "tablebase")]

use std::{env::var, error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to a test client
static TABLEBASE: LazyLock<Licheszter> = LazyLock::new(|| {
    dotenvy::dotenv().ok();

    let token = var("TEST_TOKEN").expect("TEST_TOKEN must be set for opening explorer tests");
    Licheszter::builder().with_authentication(&token).unwrap().build()
});

#[tokio::test]
async fn tablebase_standard() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase()
        .standard("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase()
        .standard("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase().standard("invalidfen").await;
    assert!(result.is_err(), "Fetching tablebase did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn tablebase_atomic() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase()
        .atomic("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase()
        .atomic("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase().atomic("invalidfen").await;
    assert!(result.is_err(), "Fetching tablebase did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn tablebase_antichess() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase()
        .antichess("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase()
        .antichess("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to get tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase().antichess("invalidfen").await;
    assert!(result.is_err(), "Fetching tablebase did not fail: {:?}", result.unwrap());
}
