use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to test account
static LICHESS: LazyLock<Licheszter> = LazyLock::new(Licheszter::new);

#[tokio::test]
async fn tv_current_games() {
    // Run some test cases
    let result = LICHESS.tv_current_games().await;
    assert!(
        result.is_ok(),
        "Failed to get current TV games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}
