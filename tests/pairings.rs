use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::{games::GameOptions, pairings::BulkPairingOptions},
    models::game::{CorrespondenceDays, Rules, VariantMode},
};

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
async fn bulk_pairings_list() {
    // Run some test cases
    let result = LI.bulk_pairings_list().await;
    assert!(
        result.is_ok(),
        "Failed to list bulk pairings: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_list().await;
    assert!(
        result.is_ok(),
        "Failed to list bulk pairings: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn bulk_pairings_create() {
    // Create options for testing
    let options1 = BulkPairingOptions::new()
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .message("{game}")
        .players(vec![("lip_bot0", "lip_bot1")])
        .rated(true)
        .rules(vec![Rules::NoRematch, Rules::NoEarlyDraw])
        .variant(VariantMode::FromPosition);
    let options2 = BulkPairingOptions::new()
        .clock(24897, 255)
        .players(vec![("lip_bot0", "lip_bot1")]);

    // Run some test cases
    let result = LI.bulk_pairings_create(&options1).await;
    assert!(
        result.is_ok(),
        "Failed to create bulk pairing: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_create(&options2).await;
    assert!(
        result.is_ok(),
        "Failed to create bulk pairing: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.bulk_pairings_create(&BulkPairingOptions::new()).await;
    assert!(result.is_err(), "Creating bulk pairing did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bulk_pairings_clocks_start() {
    // Create options and bulk pairings for testing
    let options = BulkPairingOptions::new()
        .clock(24897, 255)
        .players(vec![("lip_bot0", "lip_bot1")]);
    let bulk = LI.bulk_pairings_create(&options).await.unwrap();

    // Run some test cases
    let result = LI.bulk_pairings_clocks_start(&bulk.id).await;
    assert!(
        result.is_ok(),
        "Failed to start bulk pairing clocks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_clocks_start(&bulk.id).await;
    assert!(
        result.is_err(),
        "Starting bulk pairing clocks did not fail: {:?}",
        result.unwrap()
    );

    let result = LI.bulk_pairings_clocks_start("notvalid").await;
    assert!(
        result.is_err(),
        "Starting bulk pairing clocks did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bulk_pairings_show() {
    // Create options and bulk pairings for testing
    let options = BulkPairingOptions::new()
        .clock(24897, 255)
        .players(vec![("lip_bot0", "lip_bot1")]);
    let bulk = LI.bulk_pairings_create(&options).await.unwrap();

    // Run some test cases
    let result = LI.bulk_pairings_show(&bulk.id).await;
    assert!(
        result.is_ok(),
        "Failed to get bulk pairing: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_show(&bulk.id).await;
    assert!(result.is_err(), "Getting bulk pairing did not fail: {:?}", result.unwrap());

    let result = LI.bulk_pairings_show("notvalid").await;
    assert!(result.is_err(), "Getting bulk pairing did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bulk_pairings_cancel() {
    // Create options and bulk pairings for testing
    let options = BulkPairingOptions::new()
        .clock(24897, 255)
        .players(vec![("lip_bot0", "lip_bot1")]);
    let bulk = LI.bulk_pairings_create(&options).await.unwrap();

    // Run some test cases
    let result = LI.bulk_pairings_cancel(&bulk.id).await;
    assert!(
        result.is_ok(),
        "Failed to cancel bulk pairing: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_cancel(&bulk.id).await;
    assert!(result.is_err(), "Cancelling bulk pairing did not fail: {:?}", result.unwrap());

    let result = LI.bulk_pairings_cancel("notvalid").await;
    assert!(result.is_err(), "Cancelling bulk pairing did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bulk_pairings_export() {
    let bulk_options = BulkPairingOptions::new()
        .clock(24897, 255)
        .players(vec![("lip_bot0", "lip_bot1")]);
    let bulk = LI.bulk_pairings_create(&bulk_options).await.unwrap();
    let options = GameOptions::new()
        .moves(true)
        .tags(true)
        .clocks(true)
        .evals(true)
        .accuracy(true)
        .opening(true)
        .division(true)
        .literate(true);

    // Run some test cases
    let mut result = LI.bulk_pairings_export(&bulk.id, Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export bulk pairing: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.bulk_pairings_export(&bulk.id, None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export bulk pairing: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = BOT0.bulk_pairings_export(&bulk.id, Some(&options)).await;
    assert!(result.is_err(), "Exporting bulk pairing did not fail");

    let result = LI.bulk_pairings_export("notvalid", None).await;
    assert!(result.is_err(), "Exporting bulk pairing did not fail");
}
