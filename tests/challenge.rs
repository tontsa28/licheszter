use licheszter::{client::Licheszter, error::Result};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn challenge_create() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/challenge_create.json").await?;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/Bot0"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    let data = client.challenge_create("Bot0", None).await?;
    dbg!(data);

    Ok(())
}

#[tokio::test]
async fn challenge_accept() -> Result<()> {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/YzOF02S1/accept"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.challenge_accept("YzOF02S1").await?;

    Ok(())
}

#[tokio::test]
async fn challenge_decline() -> Result<()> {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/YzOF02S1/decline"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.challenge_decline("YzOF02S1", None).await?;

    Ok(())
}

#[tokio::test]
async fn challenge_cancel() -> Result<()> {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/YzOF02S1/cancel"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.challenge_cancel("YzOF02S1").await?;

    Ok(())
}

#[tokio::test]
async fn challenge_ai() -> Result<()> {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/challenge_ai.json").await?;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/challenge/ai"))
        .respond_with(ResponseTemplate::new(201).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    let data = client.challenge_ai(8, None).await?;
    dbg!(data);

    Ok(())
}

#[tokio::test]
async fn challenges() -> Result<()> {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/challenges.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("api/challenge"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    let data = client.challenge_list().await?;
    dbg!(data);

    Ok(())
}
