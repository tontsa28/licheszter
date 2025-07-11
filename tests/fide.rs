use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to a test client
static LICHESS: LazyLock<Licheszter> = LazyLock::new(Licheszter::new);

#[tokio::test]
async fn fide_player() {
    // Run some test cases
    let result = LICHESS.fide_player(1503014).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide_player(509825).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide_player(9999999).await;
    assert!(result.is_err(), "Fetching FIDE player did not fail: {:?}", result.unwrap());
}
