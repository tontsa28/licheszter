#![cfg(feature = "tablebase")]

use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to a test client
static TABLEBASE: LazyLock<Licheszter> = LazyLock::new(|| Licheszter::new());

#[tokio::test]
async fn tablebase_standard() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase_standard("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase_standard("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase_standard("invalidfen").await;
    assert!(
        result.is_err(),
        "Fetching tablebase did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn tablebase_atomic() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase_atomic("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase_atomic("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase_atomic("invalidfen").await;
    assert!(
        result.is_err(),
        "Fetching tablebase did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn tablebase_antichess() {
    // Run some test cases
    let result = TABLEBASE
        .tablebase_antichess("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE
        .tablebase_antichess("8/8/8/8/7P/4B3/4kP1K/8 b - - 0 46")
        .await;
    assert!(
        result.is_ok(),
        "Failed to fetch tablebase: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = TABLEBASE.tablebase_antichess("invalidfen").await;
    assert!(
        result.is_err(),
        "Fetching tablebase did not fail: {:?}",
        result.unwrap()
    );
}
