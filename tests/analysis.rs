use std::{error::Error, sync::LazyLock};

use licheszter::{client::Licheszter, models::game::VariantMode};

// Connect to a test account
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build()
});

#[tokio::test]
async fn analysis_cloud() {
    // Run some test cases
    let result = LI
        .analysis_cloud("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", None, None)
        .await;
    assert!(
        result.is_ok(),
        "Failed to get cloud analysis: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .analysis_cloud(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
            Some(5),
            Some(VariantMode::Standard),
        )
        .await;
    assert!(
        result.is_ok(),
        "Failed to get cloud analysis: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .analysis_cloud(
            "rnb1kbnr/pppp1ppp/8/4p3/5PPq/8/PPPPP2P/RNBQKBNR w KQkq - 1 3",
            Some(3),
            Some(VariantMode::Atomic),
        )
        .await;
    assert!(result.is_err(), "Getting cloud analysis did not fail: {:?}", result.unwrap());
}
