use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

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
