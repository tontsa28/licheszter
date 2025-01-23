use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{client::Licheszter, config::puzzles::PuzzleDifficulty};

// Connect to test accounts
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
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
    let result = LI.puzzle_daily().await;
    assert!(
        result.is_ok(),
        "Failed to get daily puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzle_daily().await;
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
        "Failed to get puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzle_show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_next() {
    // Run some test cases
    let result = LI.puzzle_next(None, None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .puzzle_next(Some("rookEndgame"), Some(PuzzleDifficulty::Normal))
        .await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzle_next(Some("mix"), None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzle_next(None, Some(PuzzleDifficulty::Hardest)).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzle_next(None, None).await;
    assert!(
        result.is_ok(),
        "Failed to get next puzzle: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_activity() {
    // Run some test cases
    let mut result = LI.puzzle_activity(Some(10), None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.puzzle_activity(None, Some(1704060000000)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.puzzle_activity(Some(5), Some(1704060000000)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get puzzle activity: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = DEFAULT.puzzle_activity(None, None).await;
    assert!(result.is_err(), "Getting puzzle activity did not fail");
}

#[tokio::test]
async fn puzzle_dashboard() {
    // Run some test cases
    let result = LI.puzzle_dashboard(90).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzle_dashboard(120).await;
    assert!(result.is_err(), "Getting puzzle dashboard did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn puzzle_dashboard_storm() {
    // Run some test cases
    let result = LI.puzzle_dashboard_storm("Li", Some(1)).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzle_dashboard_storm("Li", None).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.puzzle_dashboard_storm("Bot0", None).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle storm dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn puzzle_race_create() {
    // Run some test cases
    let result = LI.puzzle_race_create().await;
    assert!(
        result.is_ok(),
        "Failed to create puzzle race: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.puzzle_race_create().await;
    assert!(result.is_err(), "Creating puzzle race did not fail: {:?}", result.unwrap());
}
