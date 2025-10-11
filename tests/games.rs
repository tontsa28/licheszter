use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to test clients
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
async fn games_export_one() {
    // Run some test cases
    let result = LI.games_export_one("notvalid", None).await;
    assert!(result.is_err(), "Exporting one game did not fail");
}

#[tokio::test]
async fn games_ongoing() {
    // Run some test cases
    let result = LI.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}
