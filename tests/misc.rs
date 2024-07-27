use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::client::Licheszter;
use tokio::time::{sleep, Duration};

// Connect to test accounts
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build()
});

#[tokio::test]
async fn events_stream() {
    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = LI.events_stream().await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    sleep(Duration::from_secs(1)).await;
    thread.abort();

    let thread = tokio::spawn(async move {
        let mut result = BOT0.events_stream().await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    sleep(Duration::from_secs(1)).await;
    thread.abort();
}

#[tokio::test]
async fn games_ongoing() {
    // Run some test cases
    let result = LI.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to fetch ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to fetch ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn bots_online() {
    // Run some test cases
    let mut result = LI.bots_online(10).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to fetch online bots: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = LI.bots_online(0).await;
    assert!(result.is_ok(), "Failed to fetch 0 online bots");
}
