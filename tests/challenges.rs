use std::error::Error;

use licheszter::{
    client::Licheszter,
    config::challenges::{AIChallengeOptions, ChallengeOptions, OpenChallengeOptions},
    models::{
        board::ChallengeDeclineReason,
        game::{AILevel, Color, CorrespondenceDays, Rules, VariantMode},
    },
};

#[tokio::test]
async fn challenge_list() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create some challenges for testing
    li.challenge_create("Bot0", None).await.unwrap();
    bot0.challenge_create("Li", None).await.unwrap();

    // Run some test cases
    let result = bot0.challenge_list().await;
    assert!(
        result.is_ok(),
        "Failed to fetch Bot0 challenges: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_list().await;
    assert!(
        result.is_ok(),
        "Failed to fetch Li challenges: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn challenge_create() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    // Create options for testing
    let options = ChallengeOptions::new()
        .rated(true)
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .color(Color::Black)
        .variant(VariantMode::Antichess)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .rules(vec![Rules::NoEarlyDraw, Rules::NoRematch]);

    // Run some test cases
    let result = li.challenge_create("Bot0", None).await;
    assert!(
        result.is_ok(),
        "Failed to challenge user Bot0: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_create("Adriana", Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to challenge user Adriana: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_create("Bot0", Some(&options)).await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Challenging user Bot0 did not fail: {:?}",
        result.unwrap()
    );

    let result = li.challenge_create("NoSuchUser", None).await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Challenging user NoSuchUser did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_show() {
    // Connect a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    // Create a challenge for testing
    let challenge = li.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = li.challenge_show(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to fetch challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_show("notvalid").await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Fetching challenge did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_accept() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create a challenge for testing
    let challenge = li.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = bot0.challenge_accept(&challenge.id).await;
    assert!(
        result.is_ok(),
        "Failed to accept challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.challenge_accept("notvalid").await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Accepting challenge did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_decline() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create a challenge for testing
    let challenge = li.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = bot0
        .challenge_decline(&challenge.id, Some(ChallengeDeclineReason::OnlyBot))
        .await;
    assert!(
        result.is_ok(),
        "Failed to decline challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.challenge_decline("notvalid", None).await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Declining challenge did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_cancel() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create some challenges for testing
    let challenge1 = li.challenge_create("Bot0", None).await.unwrap();
    let challenge2 = li.challenge_create("Bot0", None).await.unwrap();

    // Run some test cases
    let result = bot0.challenge_cancel(&challenge1.id, None).await;
    assert!(
        result.is_ok(),
        "Failed to cancel challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.challenge_cancel(&challenge2.id, Some("lip_li")).await;
    assert!(
        result.is_ok(),
        "Failed to cancel challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0.challenge_cancel("notvalid", Some("notvalid")).await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Cancelling challenge did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_ai() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    // Create options for testing
    let options = AIChallengeOptions::new()
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .color(Color::Black)
        .variant(VariantMode::Antichess)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2");

    // Run some test cases
    let result = li.challenge_ai(AILevel::One, None).await;
    assert!(
        result.is_ok(),
        "Failed to challenge Lichess AI: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_ai(AILevel::Eight, Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to challenge Lichess AI: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn challenge_create_open() {
    // Connect to a test account
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    // Create options for testing
    let options = OpenChallengeOptions::new()
        .rated(true)
        .clock(24897, 255)
        .days(CorrespondenceDays::Seven)
        .fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        .name("An Open Challenge")
        .rules(vec![Rules::NoRematch, Rules::NoEarlyDraw])
        .users(vec!["Adriana", "Bot0"])
        .variant(VariantMode::Antichess);

    // Run some test cases
    let result = li.challenge_create_open(None).await;
    assert!(
        result.is_ok(),
        "Failed to create an open challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_create_open(Some(&options)).await;
    dbg!(&result);
    assert!(
        result.is_ok(),
        "Failed to create an open challenge: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn challenge_game_clocks_start() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create a game for testing
    let options = ChallengeOptions::new().clock(180, 2);
    let challenge = li.challenge_create("Bot0", Some(&options)).await.unwrap();
    bot0.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = bot0
        .challenge_game_clocks_start(&challenge.id, "lip_li", "lip_bot0")
        .await;
    assert!(
        result.is_ok(),
        "Failed to start game clocks: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0
        .challenge_game_clocks_start("notvalid", "notvalid", "notvalid")
        .await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Starting game clocks did not fail: {:?}",
        result.unwrap()
    );

    let result = bot0
        .challenge_game_clocks_start("notvalid", "lip_li", "lip_bot0")
        .await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Starting game clocks did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn challenge_opponent_clock_increment() {
    // Connect to test accounts
    let li = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build();

    let bot0 = Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build();

    // Create a game for testing
    let options = ChallengeOptions::new().clock(180, 2);
    let challenge = li.challenge_create("Bot0", Some(&options)).await.unwrap();
    bot0.challenge_accept(&challenge.id).await.unwrap();

    // Run some test cases
    let result = li
        .challenge_opponent_clock_increment(&challenge.id, 30)
        .await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = bot0
        .challenge_opponent_clock_increment(&challenge.id, 30)
        .await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li
        .challenge_opponent_clock_increment(&challenge.id, 100000)
        .await;
    assert!(
        result.is_ok(),
        "Failed to add time to opponent clock: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = li.challenge_opponent_clock_increment("notvalid", 30).await;
    assert!(
        result.as_ref().is_err_and(|err| err.is_lichess()),
        "Adding time to opponent clock did not fail: {:?}",
        result.unwrap()
    );
}
