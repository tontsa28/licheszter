#![cfg(feature = "bot")]

use std::error::Error;

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::challenges::ChallengeOptions,
    models::{board::ChatRoom, game::Color},
};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn bot_game_stream() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Connect to test accounts
    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = bot0.challenge_create("Bot1", Some(&options)).await.unwrap();
    bot1.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let mut result = bot0.bot_game_stream(&challenge.id).await.unwrap();
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
    bot0.bot_play_move(&challenge.id, "e2e4", true)
        .await
        .unwrap();
    bot1.bot_play_move(&challenge.id, "e7e5", true)
        .await
        .unwrap();
    bot0.bot_play_move(&challenge.id, "g1f3", true)
        .await
        .unwrap();
    bot1.bot_play_move(&challenge.id, "b1c3", true)
        .await
        .unwrap();

    bot0.bot_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();
    bot1.bot_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;
    thread.abort();
}

#[tokio::test]
async fn bot_play_move() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = bot0.challenge_create("Bot1", Some(&options)).await.unwrap();
    bot1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = bot0.bot_play_move(&challenge.id, "e2e4", false).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_play_move(&challenge.id, "e7e5", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_play_move(&challenge.id, "d1d3", true).await;
    assert!(
        result.is_err(),
        "Playing a move did not fail: {:?}",
        result.unwrap()
    );

    let result = bot0.bot_play_move(&challenge.id, "g1f3", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_play_move(&challenge.id, "b8c6", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_play_move("notvalid", "a1a3", false).await;
    assert!(
        result.is_err(),
        "Playing a move did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_chat_write() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create a game for testing
    let challenge = bot0.challenge_create("Bot1", None).await.unwrap();
    bot1.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = bot0
        .bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF!")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1
        .bot_chat_write(&challenge.id, ChatRoom::Spectator, "GLHF!")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0
        .bot_chat_write("notvalid", ChatRoom::Player, "GLHF!")
        .await;
    assert!(
        result.is_err(),
        "Writing to chat did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_chat_read() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create a game for testing
    let challenge = bot0.challenge_create("Bot1", None).await.unwrap();
    bot1.challenge_accept(&challenge.id).await.unwrap();

    // Write some messages to the chat
    bot0.bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();
    bot1.bot_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();

    // Run some test cases
    let result = bot0.bot_chat_read(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to read chat messages: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_chat_read("notvalid").await;
    assert!(
        result.is_err(),
        "Reading chat messages did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_game_abort() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create some games for testing
    let challenge1 = bot0.challenge_create("Adriana", None).await.unwrap();
    let challenge2 = bot0.challenge_create("Adriana", None).await.unwrap();
    bot1.challenge_accept(&challenge1.id).await.unwrap();
    bot1.challenge_accept(&challenge2.id).await.unwrap();

    // Run some test cases
    let result = bot0.bot_game_abort(&challenge1.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_game_abort(&challenge2.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_game_abort(&challenge1.id).await;
    assert!(
        result.is_err(),
        "Aborting game did not fail: {:?}",
        result.unwrap()
    );

    let result = bot0.bot_game_abort("notvalid").await;
    assert!(
        result.is_err(),
        "Aborting game did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_game_resign() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create some games for testing
    let challenge1 = bot0.challenge_create("Bot1", None).await.unwrap();
    let challenge2 = bot0.challenge_create("Bot1", None).await.unwrap();
    bot1.challenge_accept(&challenge1.id).await.unwrap();
    bot1.challenge_accept(&challenge2.id).await.unwrap();

    // Run some test cases
    let result = bot0.bot_game_resign(&challenge1.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_game_resign(&challenge2.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot1.bot_game_resign(&challenge1.id).await;
    assert!(
        result.is_err(),
        "Resigning game did not fail: {:?}",
        result.unwrap()
    );

    let result = bot0.bot_game_resign("notvalid").await;
    assert!(
        result.is_err(),
        "Resigning game did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_handle_draws() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create some games for testing
    let challenge1 = bot0.challenge_create("Bot1", None).await.unwrap();
    let challenge2 = bot0.challenge_create("Bot1", None).await.unwrap();
    bot1.challenge_accept(&challenge1.id).await.unwrap();
    bot1.challenge_accept(&challenge2.id).await.unwrap();

    // Run some test cases
    let result = bot0.bot_handle_draws(&challenge1.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_handle_draws(&challenge2.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_handle_draws("notvalid", true).await;
    assert!(
        result.is_err(),
        "Handling draws did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn bot_handle_takebacks() {
    // Connect to test accounts
    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    let bot1 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot1")
        .build();

    // Create some games for testing
    let challenge1 = bot0.challenge_create("Bot1", None).await.unwrap();
    let challenge2 = bot0.challenge_create("Bot1", None).await.unwrap();
    bot1.challenge_accept(&challenge1.id).await.unwrap();
    bot1.challenge_accept(&challenge2.id).await.unwrap();

    // Run some test cases
    let result = bot0.bot_handle_takebacks(&challenge1.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_handle_takebacks(&challenge2.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.bot_handle_takebacks("notvalid", true).await;
    assert!(
        result.is_err(),
        "Handling takebacks did not fail: {:?}",
        result.unwrap()
    );
}
