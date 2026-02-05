use std::{error::Error, sync::LazyLock};

use licheszter::client::Licheszter;

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static ADRIANA: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_adriana")
        .unwrap()
        .build()
});

#[tokio::test]
async fn simuls_current() {
    // Run some test cases
    let result = LI.simuls_current().await;
    assert!(
        result.is_ok(),
        "Failed to get current simuls: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.simuls_current().await;
    assert!(
        result.is_ok(),
        "Failed to get current simuls: {:?}",
        result.unwrap_err().source().unwrap()
    );
}
