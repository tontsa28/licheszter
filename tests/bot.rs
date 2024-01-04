use futures_util::TryStreamExt;
use licheszter::{client::Licheszter, error::Result};
use wiremock::{
    matchers::{method, path, query_param},
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
        .and(query_param("offeringDraw", "true"))
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

#[tokio::test]
async fn bot_chat_write() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/x-ndjson"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.bot_chat_write("is9Gsjun", "player", "Good luck!").await?;

    Ok(())
}

#[tokio::test]
async fn bot_game_abort() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/abort"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/x-ndjson"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.bot_game_abort("is9Gsjun").await?;

    Ok(())
}

#[tokio::test]
async fn bot_game_resign() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/resign"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/x-ndjson"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())?
        .build();

    // Call the mock
    client.bot_game_resign("is9Gsjun").await?;

    Ok(())
}

#[tokio::test]
async fn bots_online() -> Result<()> {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/bots_online.json").await?;

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("api/bot/online"))
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
    let mut stream = client.bots_online(50).await?;
    while stream.try_next().await?.is_some() {}

    Ok(())
}