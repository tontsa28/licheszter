use std::{error::Error, panic, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::tv::{TvChannel, TvChannelOptions},
};
use tokio::time::{Duration, sleep};

// Connect to test account
static LICHESS: LazyLock<Licheszter> = LazyLock::new(Licheszter::new);

#[tokio::test]
async fn tv_games() {
    // Run a test case
    let result = LICHESS.tv_games().await;
    assert!(
        result.is_ok(),
        "Failed to get current TV games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn tv_connect() {
    // Run a test case
    let thread = tokio::spawn(async move {
        let mut result = LICHESS.tv_connect().await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    sleep(Duration::from_secs(3)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }
}

#[tokio::test]
async fn tv_channel_connect() {
    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = LICHESS.tv_channel_connect(TvChannel::Bullet).await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    sleep(Duration::from_secs(3)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }

    let thread = tokio::spawn(async move {
        let mut result = LICHESS.tv_channel_connect(TvChannel::Bot).await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });
    sleep(Duration::from_secs(3)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }
}

#[tokio::test]
async fn tv_channel_games() {
    // Create options for testing
    let options = TvChannelOptions::new()
        .amount(5)
        .moves(true)
        .tags(true)
        .clocks(true)
        .opening(true);

    // Run some test cases
    let mut result = LICHESS.tv_channel_games(TvChannel::Bullet, None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to parse an event: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LICHESS
        .tv_channel_games(TvChannel::Bullet, Some(&options))
        .await
        .unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to parse an event: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }
}
