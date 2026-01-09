use std::{error::Error, sync::LazyLock};

use licheszter::{
    client::Licheszter,
    config::pairings::BulkPairingOptions,
    models::game::{CorrespondenceDays, Rules, VariantMode},
};

// Connect to test clients
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
        "Failed to start bulk pairing clocks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bulk_pairings_cancel(&bulk.id).await;
    assert!(
        result.is_err(),
        "Starting bulk pairing clocks did not fail: {:?}",
        result.unwrap()
    );

    let result = LI.bulk_pairings_cancel("notvalid").await;
    assert!(
        result.is_err(),
        "Starting bulk pairing clocks did not fail: {:?}",
        result.unwrap()
    );
}
