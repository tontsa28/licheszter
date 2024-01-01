use futures_util::TryStreamExt;
use licheszter::{client::Licheszter, error::Result};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn bot_game_stream() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/bot_game_stream.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("api/bot/game/stream/is9Gsjun"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(response, "application/x-ndjson")
                .append_header("Transfer-Encoding", "chunked"),
        )
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    let mut stream = client.bot_game_stream("is9Gsjun").await?;
    while stream.try_next().await?.is_some() {}

    Ok(())
}

#[tokio::test]
async fn bot_play_move() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/move/e2e4"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/x-ndjson"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.bot_play_move("is9Gsjun", "e2e4", true).await?;

    Ok(())
}
