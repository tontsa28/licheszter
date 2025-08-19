#![cfg(feature = "bot")]

use std::{error::Error, panic, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::challenges::ChallengeOptions,
    models::{chat::ChatRoom, game::Color},
};
use tokio::time::{sleep, Duration};

// Connect to test accounts
static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build()
});

static BOT1: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build()
});

#[tokio::test]
async fn bot_game_connect() {
    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = BOT0.challenge_create("Bot1", Some(&options)).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let mut result = BOT0.bot_game_connect(&challenge.id).await.unwrap();
    let thread = tokio::spawn(async move {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to parse an event: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    // Play the game
    BOT0.bot_play_move(&challenge.id, "e2e4", true).await.unwrap();
    BOT1.bot_play_move(&challenge.id, "e7e5", true).await.unwrap();
    BOT0.bot_play_move(&challenge.id, "g1f3", true).await.unwrap();
    BOT1.bot_play_move(&challenge.id, "b1c3", true).await.unwrap();

    BOT0.bot_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();
    BOT1.bot_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;
    thread.abort();
    let result = thread.await;
    if result.as_ref().is_err_and(|e| e.is_panic()) {
        panic::resume_unwind(result.unwrap_err().into_panic());
    }
}

#[tokio::test]
async fn bot_play_move() {
    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = BOT0.challenge_create("Bot1", Some(&options)).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_play_move(&challenge.id, "e2e4", false).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_play_move(&challenge.id, "e7e5", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_play_move(&challenge.id, "d1d3", true).await;
    assert!(result.is_err(), "Playing a move did not fail: {:?}", result.unwrap());

    let result = BOT0.bot_play_move(&challenge.id, "g1f3", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_play_move(&challenge.id, "b8c6", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_play_move("notvalid", "a1a3", false).await;
    assert!(result.is_err(), "Playing a move did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_chat_write() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0
        .bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF!")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1
        .bot_chat_write(&challenge.id, ChatRoom::Spectator, "GLHF!")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_chat_write("notvalid", ChatRoom::Player, "GLHF!").await;
    assert!(result.is_err(), "Writing to chat did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_chat_read() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Write some messages to the chat
    BOT0.bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();
    BOT1.bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();

    // Run some test cases
    let result = BOT0.bot_chat_read(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to read chat messages: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_chat_read("notvalid").await;
    assert!(result.is_err(), "Reading chat messages did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_game_abort() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_game_abort(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_game_abort(&challenge.id).await;
    assert!(result.is_err(), "Aborting game did not fail: {:?}", result.unwrap());

    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT1.bot_game_abort(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_game_abort("notvalid").await;
    assert!(result.is_err(), "Aborting game did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_game_resign() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_game_resign(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_game_resign(&challenge.id).await;
    assert!(result.is_err(), "Resigning game did not fail: {:?}", result.unwrap());

    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT1.bot_game_resign(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_game_resign("notvalid").await;
    assert!(result.is_err(), "Resigning game did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_handle_draws() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let result = BOT0.bot_handle_draws(&challenge.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_handle_draws(&challenge.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_handle_draws("notvalid", true).await;
    assert!(result.is_err(), "Handling draws did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_handle_takebacks() {
    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let result = BOT0.bot_handle_takebacks(&challenge.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Create a game for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_handle_takebacks(&challenge.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_handle_takebacks("notvalid", true).await;
    assert!(result.is_err(), "Handling takebacks did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn bot_claim_victory() {
    // Create some games for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_claim_victory(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim victory of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_claim_victory(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim victory of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_claim_victory("notvalid").await;
    assert!(
        result.is_err(),
        "Claiming victory of a game did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_claim_draw() {
    // Create some games for testing
    let challenge = BOT0.challenge_create("Bot1", None).await.unwrap();
    BOT1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = BOT0.bot_claim_draw(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim draw of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT1.bot_claim_draw(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim draw of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.bot_claim_draw("notvalid").await;
    assert!(result.is_err(), "Claiming draw of a game did not fail: {:?}", result.unwrap());
}
