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
async fn puzzles_daily() {
    // Run some test cases
    let result = LI.puzzles_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.puzzles_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().puzzles_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}