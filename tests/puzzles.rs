#![cfg(feature = "puzzles")]

use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{client::Licheszter, config::puzzles::PuzzleDifficulty};

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static DEFAULT: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .build()
});

#[tokio::test]
async fn puzzle_daily() {
    // Run some test cases
    let result = LI.puzzles().daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzles().daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_show() {
    // Get a puzzle ID for testing
    let id = LI.puzzles().daily().await.unwrap().puzzle.id;

    // Run some test cases
    let result = LI.puzzles().show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzles().show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_next() {
    // Run some test cases
    let result = LI.puzzles().next(None, None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .puzzles()
        .next(Some("rookEndgame"), Some(PuzzleDifficulty::Normal))
        .await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzles().next(Some("mix"), None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .puzzles()
        .next(None, Some(PuzzleDifficulty::Hardest))
        .await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzles().next(None, None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_activity() {
    // Run some test cases
    let mut result = LI.puzzles().activity(Some(10), None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI
        .puzzles()
        .activity(None, Some(1704060000000))
        .await
        .unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI
        .puzzles()
        .activity(Some(5), Some(1704060000000))
        .await
        .unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = DEFAULT.puzzles().activity(None, None).await;
    assert!(result.is_err(), "Getting puzzle activity did not fail");
}

#[tokio::test]
async fn puzzle_dashboard() {
    // Run some test cases
    let result = LI.puzzles().dashboard(90).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzles().dashboard(120).await;
    assert!(
        result.is_err(),
        "Getting puzzle dashboard did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn puzzle_dashboard_storm() {
    // Run some test cases
    let result = LI.puzzles().dashboard_storm("Li", Some(1)).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzles().dashboard_storm("Li", None).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzles().dashboard_storm("Bot0", None).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_race_create() {
    // Run some test cases
    let result = LI.puzzles().race_create().await;
    assert!(
        result.is_ok(),
        "Failed to create puzzle race: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzles().race_create().await;
    assert!(
        result.is_err(),
        "Creating puzzle race did not fail: {:?}",
        result.unwrap()
    );
}
