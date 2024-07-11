use std::error::Error;

use licheszter::{
    client::Licheszter,
    config::challenges::ChallengeCreateOptions,
    models::{
        board::ChallengeDeclineReason,
        game::{Color, CorrespondenceDays, Rules, VariantMode},
    },
};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
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
    let options = ChallengeCreateOptions::new()
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
        result.is_err(),
        "Challenging user Bot0 did not fail: {:?}",
        result.unwrap()
    );

    let result = li.challenge_create("NoSuchUser", None).await;
    assert!(
        result.is_err(),
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
    assert!(result.is_ok(), "Failed to fetch challenge: {:?}", result.unwrap_err().source().unwrap());

    let result = li.challenge_show("notvalid").await;
    assert!(result.is_err(), "Fetching challenge did not fail: {:?}", result.unwrap());
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
    assert!(result.is_ok(), "Failed to accept challenge: {:?}", result.unwrap_err().source().unwrap());

    let result = bot0.challenge_accept("notvalid").await;
    assert!(result.is_err(), "Accepting challenge did not fail: {:?}", result.unwrap());
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
    assert!(result.is_ok(), "Failed to decline challenge: {:?}", result.unwrap_err().source().unwrap());

    let result = bot0.challenge_decline("notvalid", None).await;
    assert!(result.is_err(), "Declining challenge did not fail: {:?}", result.unwrap());
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
    assert!(result.is_ok(), "Failed to cancel challenge: {:?}", result.unwrap_err().source().unwrap());

    let result = bot0.challenge_cancel(&challenge2.id, Some("lip_li")).await;
    assert!(result.is_ok(), "Failed to cancel challenge: {:?}", result.unwrap_err().source().unwrap());

    let result = bot0.challenge_cancel("notvalid", Some("notvalid")).await;
    assert!(result.is_err(), "Cancelling challenge did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn challenge_ai() {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/challenge_ai.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/ai"))
        .respond_with(ResponseTemplate::new(201).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.challenge_ai(8, None).await.unwrap();
}
