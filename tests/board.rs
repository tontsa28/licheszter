#![cfg(feature = "board")]

use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::{board::SeekOptions, challenges::ChallengeOptions},
    models::{
        board::ChatRoom,
        game::{Color, VariantMode},
    },
};
use tokio::time::{sleep, Duration};

// Connect to test accounts
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
async fn board_seek_create() {
    // Create options for testing
    let options1 = SeekOptions::new()
        .rated(true)
        .clock(10, 5)
        .variant(VariantMode::Standard)
        .rating_range(0, 3000);
    let options2 = options1.to_owned();

    // Run some test cases
    let thread1 = tokio::spawn(async move {
        let mut result = LI.board_seek_create(Some(&options1)).await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to seek for a game: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    let thread2 = tokio::spawn(async move {
        let mut result = ADRIANA.board_seek_create(Some(&options2)).await.unwrap();
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to seek for a game: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    });

    tokio::try_join!(thread1, thread2).unwrap();
}

#[tokio::test]
async fn board_game_stream() {
    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = LI
        .challenge_create("Adriana", Some(&options))
        .await
        .unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let mut result = LI.board_game_stream(&challenge.id).await.unwrap();
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
    LI.board_play_move(&challenge.id, "e2e4", true)
        .await
        .unwrap();
    ADRIANA
        .board_play_move(&challenge.id, "e7e5", true)
        .await
        .unwrap();
    LI.board_play_move(&challenge.id, "g1f3", true)
        .await
        .unwrap();
    ADRIANA
        .board_play_move(&challenge.id, "b1c3", true)
        .await
        .unwrap();

    LI.board_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();
    ADRIANA
        .board_chat_write(&challenge.id, ChatRoom::Player, "Good game!")
        .await
        .unwrap();

    sleep(Duration::from_secs(1)).await;
    thread.abort();
}

#[tokio::test]
async fn board_play_move() {
    // Create a game for testing
    let options = ChallengeOptions::new().color(Color::White);
    let challenge = LI
        .challenge_create("Adriana", Some(&options))
        .await
        .unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_play_move(&challenge.id, "e2e4", false).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.board_play_move(&challenge.id, "e7e5", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_play_move(&challenge.id, "d1d3", true).await;
    assert!(
        result.is_err(),
        "Playing a move did not fail: {:?}",
        result.unwrap()
    );

    let result = LI.board_play_move(&challenge.id, "g1f3", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.board_play_move(&challenge.id, "b8c6", true).await;
    assert!(
        result.is_ok(),
        "Failed to play a move: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_play_move("notvalid", "a1a3", false).await;
    assert!(
        result.is_err(),
        "Playing a move did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_chat_write() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI
        .board_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA
        .board_chat_write(&challenge.id, ChatRoom::Spectator, "GLHF!")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to chat: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .board_chat_write("notvalid", ChatRoom::Player, "GLHF!")
        .await;
    assert!(
        result.is_err(),
        "Writing to chat did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_chat_read() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Write some messages to the chat
    LI.board_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();
    ADRIANA
        .board_chat_write(&challenge.id, ChatRoom::Player, "GLHF")
        .await
        .unwrap();

    // Run some test cases
    let result = LI.board_chat_read(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to read chat messages: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_chat_read("notvalid").await;
    assert!(
        result.is_err(),
        "Reading chat messages did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_game_abort() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_game_abort(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.board_game_abort(&challenge.id).await;
    assert!(
        result.is_err(),
        "Aborting game did not fail: {:?}",
        result.unwrap()
    );

    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = ADRIANA.board_game_abort(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to abort game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_game_abort("notvalid").await;
    assert!(
        result.is_err(),
        "Aborting game did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_game_resign() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_game_resign(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.board_game_resign(&challenge.id).await;
    assert!(
        result.is_err(),
        "Resigning game did not fail: {:?}",
        result.unwrap()
    );

    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = ADRIANA.board_game_resign(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to resign game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_game_resign("notvalid").await;
    assert!(
        result.is_err(),
        "Resigning game did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_handle_draws() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let result = LI.board_handle_draws(&challenge.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_handle_draws(&challenge.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle draws: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_handle_draws("notvalid", true).await;
    assert!(
        result.is_err(),
        "Handling draws did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_handle_takebacks() {
    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run a test case
    let result = LI.board_handle_takebacks(&challenge.id, true).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    // Create a game for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_handle_takebacks(&challenge.id, false).await;
    assert!(
        result.is_ok(),
        "Failed to handle takebacks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_handle_takebacks("notvalid", true).await;
    assert!(
        result.is_err(),
        "Handling takebacks did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn board_claim_victory() {
    // Create some games for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_claim_victory(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim victory of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = ADRIANA.board_claim_victory(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to claim victory of a game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_claim_victory("notvalid").await;
    assert!(
        result.is_err(),
        "Claiming victory of a game did not fail: {:?}",
        result.unwrap()
    );
}

// TODO: Needs more test cases when tournament functionality is implemented
#[tokio::test]
async fn board_berserk() {
    // Create some games for testing
    let challenge = LI.challenge_create("Adriana", None).await.unwrap();
    ADRIANA.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = LI.board_berserk(&challenge.id).await;
    assert!(
        result.is_err(),
        "Berserking a game did not fail: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.board_berserk("notvalid").await;
    assert!(
        result.is_err(),
        "Berserking a game did not fail: {:?}",
        result.unwrap()
    );
}
