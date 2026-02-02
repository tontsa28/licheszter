use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::challenges::{AIChallengeOptions, ChallengeOptions, OpenChallengeOptions},
    models::{
        challenge::{ChallengeComplete, ChallengeDeclineReason},
        game::{AILevel, Color, CorrespondenceDays, Rules, VariantMode},
    },
};
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
async fn challenge_list() {
    // Create some challenges for testing
    LI.challenge_create("Bot0", None).await.unwrap();
    BOT0.challenge_create("Li", None).await.unwrap();

    // Run some test cases
    let result = BOT0.challenge_list().await;
    assert!(
        result.is_ok(),
        "Failed to get challenges: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_list().await;
    assert!(
        result.is_ok(),
        "Failed to get challenges: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn challenge_create() {
    // Create options for testing
    let options = ChallengeOptions::new()
        .rated(false)
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .color(Color::Black)
        .variant(VariantMode::FromPosition)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .rules(vec![Rules::NoEarlyDraw, Rules::NoRematch]);

    // Run some test cases
    let result = LI.challenge_create("Bot0", None).await;
    assert!(
        result.is_ok(),
        "Failed to create a challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_create("Adriana", Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to create a challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_create("Bot0", Some(&options)).await;
    assert!(result.is_err(), "Creating a challenge did not fail: {:?}", result.unwrap());

    let result = LI.challenge_create("NoSuchUser", None).await;
    assert!(result.is_err(), "Creating a challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_create_connect() {
    // Create options for testing
    let options = ChallengeOptions::new()
        .rated(false)
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .color(Color::Black)
        .variant(VariantMode::FromPosition)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");

    // Run some test cases
    let mut result = LI.challenge_create_connect("Bot0", None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to create a streamed challenge: {:?}",
            event.unwrap_err().source().unwrap()
        );
        sleep(Duration::from_secs(1)).await;
        if let ChallengeComplete::Challenge(challenge) = event.unwrap() {
            BOT0.challenge_accept(&challenge.id).await.unwrap();
        }
    }

    let mut result = LI.challenge_create_connect("Bot0", Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to create a streamed challenge: {:?}",
            event.unwrap_err().source().unwrap()
        );
        sleep(Duration::from_secs(1)).await;
        if let ChallengeComplete::Challenge(challenge) = event.unwrap() {
            BOT0.challenge_accept(&challenge.id).await.unwrap();
        }
    }

    let result = LI.challenge_create_connect("Li", None).await;
    assert!(result.is_err(), "Creating a streamed challenge did not fail");

    let result = LI.challenge_create_connect("NoSuchUser", None).await;
    assert!(result.is_err(), "Creating a streamed challenge did not fail");
}

#[tokio::test]
async fn challenge_show() {
    // Create a challenge for testing
    let challenge = LI.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = LI.challenge_show(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to get challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_show("notvalid").await;
    assert!(result.is_err(), "Fetching challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_accept() {
    // Create a challenge for testing
    let challenge = LI.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = BOT0.challenge_accept(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to accept challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.challenge_accept("notvalid").await;
    assert!(result.is_err(), "Accepting challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_decline() {
    // Create a challenge for testing
    let challenge = LI.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = BOT0
        .challenge_decline(&challenge.id, Some(ChallengeDeclineReason::OnlyBot))
        .await;
    assert!(
        result.is_ok(),
        "Failed to decline challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.challenge_decline("notvalid", None).await;
    assert!(result.is_err(), "Declining challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_cancel() {
    // Create a challenge for testing
    let challenge = LI.challenge_create("Bot0", None).await.unwrap();

    // Run a test case
    let result = BOT0.challenge_cancel(&challenge.id, None).await;
    assert!(
        result.is_ok(),
        "Failed to cancel challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Create a challenge for testing
    let challenge = LI.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = BOT0.challenge_cancel(&challenge.id, Some("lip_li")).await;
    assert!(
        result.is_ok(),
        "Failed to cancel challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.challenge_cancel("notvalid", Some("notvalid")).await;
    assert!(result.is_err(), "Cancelling challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_ai() {
    // Create options for testing
    let options = AIChallengeOptions::new()
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .color(Color::Black)
        .variant(VariantMode::FromPosition)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");

    // Run some test cases
    let result = LI.challenge_ai(AILevel::One, None).await;
    assert!(
        result.is_ok(),
        "Failed to challenge Lichess AI: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_ai(AILevel::Eight, Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to challenge Lichess AI: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn challenge_create_open() {
    // Create options for testing
    let options = OpenChallengeOptions::new()
        .rated(true)
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .name("An Open Challenge")
        .rules(vec![Rules::NoRematch, Rules::NoEarlyDraw])
        .users(vec!["Adriana", "Bot0"])
        .variant(VariantMode::FromPosition);

    // Run some test cases
    let result = LI.challenge_create_open(None).await;
    assert!(
        result.is_ok(),
        "Failed to create an open challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_create_open(Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to create an open challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let options = options.users(vec!["Adriana", "Bot0", "NoSuchUser"]);
    let result = LI.challenge_create_open(Some(&options)).await;
    assert!(
        result.is_err(),
        "Creating an open challenge did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_game_clocks_start() {
    // Create a game for testing
    let options = ChallengeOptions::new().clock(180, 2);
    let challenge = LI.challenge_create("Bot0", Some(&options)).await.unwrap();
    BOT0.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0
        .challenge_game_clocks_start(&challenge.id, "lip_li", "lip_bot0")
        .await;
    assert!(
        result.is_ok(),
        "Failed to start game clocks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0
        .challenge_game_clocks_start("notvalid", "notvalid", "notvalid")
        .await;
    assert!(result.is_err(), "Starting game clocks did not fail: {:?}", result.unwrap());

    let result = BOT0
        .challenge_game_clocks_start("notvalid", "lip_li", "lip_bot0")
        .await;
    assert!(result.is_err(), "Starting game clocks did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_opponent_clock_increment() {
    // Create a game for testing
    let options = ChallengeOptions::new().clock(180, 2);
    let challenge = LI.challenge_create("Bot0", Some(&options)).await.unwrap();
    BOT0.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.challenge_opponent_clock_increment(&challenge.id, 30).await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.challenge_opponent_clock_increment(&challenge.id, 30).await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_opponent_clock_increment(&challenge.id, 100000).await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.challenge_opponent_clock_increment("notvalid", 30).await;
    assert!(
        result.is_err(),
        "Adding time to opponent clock did not fail: {:?}",
        result.unwrap()
    );
}
