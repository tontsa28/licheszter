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

#[tokio::test]
async fn fide_search() {
    // Run some test cases
    let result = LICHESS.fide_search("Carlsen").await;
    assert!(
        result.is_ok(),
        "Failed to search for FIDE players: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide_search("keinanen").await;
    assert!(
        result.is_ok(),
        "Failed to search for FIDE players: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide_search("nosuchname").await;
    assert!(
        result.as_ref().is_ok_and(|vec| vec.is_empty()),
        "Searching for FIDE players did not fail: {:?}",
        result.unwrap()
    );
}
