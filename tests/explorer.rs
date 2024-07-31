#![cfg(feature = "explorer")]

use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::explorer::{LichessOpeningOptions, MastersOpeningOptions, PlayerOpeningOptions},
    models::{
        explorer::OpeningRatings,
        game::{Color, GameType, Speed, VariantMode},
    },
};
use tokio::time::{sleep, Duration};

// Connect to a test client
static EXPLORER: LazyLock<Licheszter> = LazyLock::new(|| Licheszter::new());

#[tokio::test]
async fn opening_explorer_masters() {
    // Create options for testing
    let options = MastersOpeningOptions::new()
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(vec!["g1f3"])
        .since(1967)
        .until(2024)
        .moves(20)
        .top_games(10);

    // Run some test cases
    let result = EXPLORER.opening_explorer_masters(None).await;
    assert!(
        result.is_ok(),
        "Failed to fetch masters openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = EXPLORER.opening_explorer_masters(Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to fetch masters openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let options = options.play(vec!["d1d3"]);
    let result = EXPLORER.opening_explorer_masters(Some(&options)).await;
    assert!(
        result.is_err(),
        "Fetching masters openings did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn opening_explorer_lichess() {
    // Create options for testing
    let options = LichessOpeningOptions::new()
        .variant(VariantMode::Standard)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(vec!["g1f3"])
        .speeds(vec![Speed::Blitz, Speed::Rapid])
        .ratings(vec![OpeningRatings::TwoThousand])
        .since("1967-01")
        .until("2024-01")
        .moves(20)
        .top_games(1)
        .recent_games(1)
        .history(true);

    // Run some test cases
    let result = EXPLORER.opening_explorer_lichess(None).await;
    assert!(
        result.is_ok(),
        "Failed to fetch Lichess openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = EXPLORER.opening_explorer_lichess(Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to fetch Lichess openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let options = options.since("invalid-month");
    let result = EXPLORER.opening_explorer_lichess(Some(&options)).await;
    assert!(
        result.is_err(),
        "Fetching Lichess openings did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn opening_explorer_player() {
    // Create options for testing
    let options1 = PlayerOpeningOptions::new()
        .variant(VariantMode::Standard)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(vec!["g1f3"])
        .speeds(vec![Speed::Blitz, Speed::Rapid])
        .mode(GameType::Rated)
        .since("1967-01")
        .until("2024-01")
        .moves(20)
        .recent_games(1);
    let options2 = options1.to_owned().since("invalid-month");

    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = EXPLORER
            .opening_explorer_player("Cheszter", Color::White, None)
            .await
            .unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to fetch player openings: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    let handle = thread.abort_handle();
    sleep(Duration::from_secs(1)).await;
    handle.abort();

    let thread = tokio::spawn(async move {
        let mut result = EXPLORER
            .opening_explorer_player("Cheszter", Color::White, Some(&options1))
            .await
            .unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to fetch player openings: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    let handle = thread.abort_handle();
    sleep(Duration::from_secs(1)).await;
    handle.abort();

    let result = EXPLORER
        .opening_explorer_player("NoSuchUser", Color::Black, Some(&options2))
        .await;
    assert!(result.is_err(), "Fetching player openings did not fail");
}
