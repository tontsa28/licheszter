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

static ADRIANA: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_adriana")
        .build()
});

#[tokio::test]
async fn message_private_send() {
    // Run some test cases
    let result = LI.message_private_send("Adriana", "What's up bro?").await;
    assert!(
        result.is_ok(),
        "Failed to send private message: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.message_private_send("Li", "I'm great hbu?").await;
    assert!(
        result.is_ok(),
        "Failed to send private message: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.message_private_send("NoSuchUser", "Let's try our luck").await;
    assert!(result.is_err(), "Sending private message did not fail: {:?}", result.unwrap());

    let result = LI.message_private_send("Bot0", "Let's try our luck").await;
    assert!(result.is_err(), "Sending private message did not fail: {:?}", result.unwrap());
}
