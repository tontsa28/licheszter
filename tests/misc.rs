use std::error::Error;

use futures_util::StreamExt;
use licheszter::client::Licheszter;

#[tokio::test]
async fn events_stream() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Run some test cases
    let thread = tokio::spawn(async move {
        let mut result = li.events_stream().await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    thread.abort();

    let thread = tokio::spawn(async move {
        let mut result = bot0.events_stream().await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    thread.abort();
}

#[tokio::test]
async fn games_ongoing() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Run some test cases
    let result = li.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to fetch ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to fetch ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn bots_online() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    // Run some test cases
    let mut result = li.bots_online(10).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to fetch online bots: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = li.bots_online(0).await;
    assert!(
        result.is_ok(),
        "Failed to fetch 0 online bots"
    );
}
