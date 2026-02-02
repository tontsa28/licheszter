use std::{error::Error, panic, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::client::Licheszter;
use tokio::time::{sleep, Duration};

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .unwrap()
        .build()
});

#[tokio::test]
async fn connect() {
    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = LI.connect().await.unwrap();
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
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }

    let thread = tokio::spawn(async move {
        let mut result = BOT0.connect().await.unwrap();
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
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }
}

#[tokio::test]
async fn bots_online() {
    // Run some test cases
    let mut result = LI.bots_online(10).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get online bots: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = LI.bots_online(0).await;
    assert!(result.is_ok(), "Failed to get 0 online bots");
}
