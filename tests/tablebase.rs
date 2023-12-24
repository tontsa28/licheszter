use licheszter::{client::Licheszter, error::Result};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn tablebase_standard() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/tablebase_standard.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("standard"))
        .and(query_param("fen", "4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_tablebase_url(mock_server.uri())?
        .build();

    // Call the mock
    client
        .endgame_standard("4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1")
        .await?;

    Ok(())
}

#[tokio::test]
async fn tablebase_atomic() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/tablebase_atomic.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("atomic"))
        .and(query_param("fen", "4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_tablebase_url(mock_server.uri())?
        .build();

    // Call the mock
    client
        .endgame_atomic("4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1")
        .await?;

    Ok(())
}

#[tokio::test]
async fn tablebase_antichess() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/tablebase_antichess.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("antichess"))
        .and(query_param("fen", "4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_tablebase_url(mock_server.uri())?
        .build();

    // Call the mock
    client
        .endgame_antichess("4k3/6KP/8/8/8/8/7p/8_w_-_-_0_1")
        .await?;

    Ok(())
}
