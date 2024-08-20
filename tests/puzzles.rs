use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::client::Licheszter;

// Connect to test accounts
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
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

    let result = Licheszter::new().puzzle_daily().await;
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

    let result = Licheszter::new().puzzle_show(&id).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle: {:?}",
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

    let result = Licheszter::new().puzzle_activity(None, None).await;
    assert!(result.is_err(), "Getting puzzle activity did not fail");
}

#[tokio::test]
async fn puzzle_dashboard() {
    // Run some test cases
    let result = LI.puzzle_dashboard(10).await;
    assert!(
        result.is_ok(),
        "Failed to get puzzle dashboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().puzzle_dashboard(30).await;
    assert!(
        result.is_err(),
        "Getting puzzle dashboard did not fail: {:?}",
        result.unwrap()
    );
}
