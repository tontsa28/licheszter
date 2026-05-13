#![cfg(feature = "fide")]

use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to a test client
static LICHESS: LazyLock<Licheszter> = LazyLock::new(Licheszter::new);

#[tokio::test]
async fn fide_player() {
    // Run some test cases
    let result = LICHESS.fide().player(1503014).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().player(509825).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE player: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().player(9999999).await;
    assert!(
        result.is_err(),
        "Fetching FIDE player did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn fide_rating_history() {
    // Run some test cases
    let result = LICHESS.fide().rating_history(1503014).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().rating_history(509825).await;
    assert!(
        result.is_ok(),
        "Failed to fetch FIDE rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().rating_history(999999).await;
    assert!(
        result.as_ref().is_ok_and(|ratings| ratings.blitz.is_empty()
            && ratings.rapid.is_empty()
            && ratings.standard.is_empty()),
        "Fetching FIDE rating history did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn fide_search() {
    // Run some test cases
    let result = LICHESS.fide().search("Carlsen").await;
    assert!(
        result.is_ok(),
        "Failed to search for FIDE players: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().search("keinanen").await;
    assert!(
        result.is_ok(),
        "Failed to search for FIDE players: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LICHESS.fide().search("nosuchname").await;
    assert!(
        result.as_ref().is_ok_and(|vec| vec.is_empty()),
        "Searching for FIDE players did not fail: {:?}",
        result.unwrap()
    );
}
