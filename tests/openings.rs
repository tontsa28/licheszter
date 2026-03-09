#![cfg(feature = "openings")]

use std::{env::var, error::Error, panic, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::openings::{LichessOpeningsOptions, MastersOpeningsOptions, PlayerOpeningsOptions},
    models::{
        common::Color,
        game::{GameType, Speed, VariantMode},
        openings::OpeningRatings,
    },
};
use tokio::time::{sleep, Duration};

// Connect to a test client
static EXPLORER: LazyLock<Licheszter> = LazyLock::new(|| {
    dotenvy::dotenv().ok();

    let token = var("TEST_TOKEN").expect("TEST_TOKEN must be set for opening explorer tests");
    Licheszter::builder()
        .with_authentication(&token)
        .unwrap()
        .build()
});

#[tokio::test]
async fn openings_masters() {
    // Create options for testing
    let options = MastersOpeningsOptions::new()
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(&["g1f3"])
        .since(1967)
        .until(2024)
        .moves(20)
        .top_games(10);

    // Run some test cases
    let result = EXPLORER.openings().masters(None).await;
    assert!(
        result.is_ok(),
        "Failed to get masters openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = EXPLORER.openings().masters(Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to get masters openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let options = options.play(&["d1d3"]);
    let result = EXPLORER.openings().masters(Some(&options)).await;
    assert!(
        result.is_err(),
        "Fetching masters openings did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn openings_lichess() {
    // Create options for testing
    let options = LichessOpeningsOptions::new()
        .variant(VariantMode::Standard)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(&["g1f3"])
        .speeds(&[Speed::Blitz, Speed::Rapid])
        .ratings(&[OpeningRatings::TwoThousand])
        .since("1967-01")
        .until("2024-01")
        .moves(20)
        .top_games(1)
        .recent_games(1)
        .history(true);

    // Run some test cases
    let result = EXPLORER.openings().lichess(None).await;
    assert!(
        result.is_ok(),
        "Failed to get Lichess openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = EXPLORER.openings().lichess(Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to get Lichess openings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let options = options.since("invalid-month");
    let result = EXPLORER.openings().lichess(Some(&options)).await;
    assert!(
        result.is_err(),
        "Fetching Lichess openings did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn openings_player() {
    // Create options for testing
    let options1 = PlayerOpeningsOptions::new()
        .variant(VariantMode::Standard)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .play(&["g1f3"])
        .speeds(&[Speed::Blitz, Speed::Rapid])
        .mode(GameType::Rated)
        .since("1967-01")
        .until("2024-01")
        .moves(20)
        .recent_games(1);
    let options2 = options1.to_owned().since("invalid-month");

    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = EXPLORER
            .openings()
            .player("Cheszter", Color::White, None)
            .await
            .unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to get player openings: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    sleep(Duration::from_secs(1)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }

    let thread = tokio::spawn(async move {
        let mut result = EXPLORER
            .openings()
            .player("Cheszter", Color::White, Some(&options1))
            .await
            .unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to get player openings: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    sleep(Duration::from_secs(1)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }

    let result = EXPLORER
        .openings()
        .player("NoSuchUser", Color::Black, Some(&options2))
        .await;
    assert!(result.is_err(), "Fetching player openings did not fail");
}

#[tokio::test]
async fn openings_masters_otb_game() {
    // Run some test cases
    let result = EXPLORER.openings().masters_otb_game("aAbqI4ey").await;
    assert!(
        result.is_ok(),
        "Failed to get masters OTB game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = EXPLORER.openings().masters_otb_game("notvalid").await;
    assert!(
        result.is_err(),
        "Getting masters OTB game did not fail: {:?}",
        result.unwrap()
    );
}
