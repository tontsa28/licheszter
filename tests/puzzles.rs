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
async fn puzzle_daily() {
    // Run some test cases
    let result = LI.puzzle_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.puzzle_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().puzzle_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_show() {
    // Get a puzzle ID for testing
    let id = LI.puzzle_daily().await.unwrap().puzzle.id;

    // Run some test cases
    let result = LI.puzzle_show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.puzzle_show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Run some test cases
    let result = Licheszter::new().puzzle_show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}