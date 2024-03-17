use futures_util::TryStreamExt;
use licheszter::{client::Licheszter, models::board::ChatRoom};
use wiremock::{
    matchers::{any, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn bot_game_stream() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/bot_game_stream.json")
        .await
        .unwrap();
    let error_response = r#"{"error":"something went wrong"}"#;

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

    Mock::given(method("GET"))
        .and(any())
        .respond_with(ResponseTemplate::new(404).set_body_raw(error_response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    while client.bot_game_stream("is9Gsjun").await.unwrap().try_next().await.unwrap().is_some() {}
    //while client.bot_game_stream("o23JsuHn").await.unwrap().try_next().await.unwrap().is_some() {}
}

#[tokio::test]
async fn bot_play_move() {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;
    let error_response = r#"{"error":"something went wrong"}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/move/e2e4"))
        .and(query_param("offeringDraw", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(400).set_body_raw(error_response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client
        .bot_play_move("is9Gsjun", "e2e4", true)
        .await
        .unwrap();
    client
        .bot_play_move("o2J3suHn", "c2c4", false)
        .await
        .unwrap_err();
}

#[tokio::test]
async fn bot_chat_write() {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;
    let error_response = r#"{"error":"something went wrong"}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(400).set_body_raw(error_response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client
        .bot_chat_write("is9Gsjun", ChatRoom::Player, "Good luck!")
        .await
        .unwrap();
    client
        .bot_chat_write("o2J3suHn", ChatRoom::Spectator, "Good game!")
        .await
        .unwrap_err();
}

#[tokio::test]
async fn bot_game_abort() {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;
    let error_response = r#"{"error":"something went wrong"}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/abort"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(400).set_body_raw(error_response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.bot_game_abort("is9Gsjun").await.unwrap();
    client.bot_game_abort("o2J3suHn").await.unwrap_err();
}

#[tokio::test]
async fn bot_game_resign() {
    // Start the mock server & set up the response
    let mock_server = MockServer::start().await;
    let response = r#"{"ok":true}"#;
    let error_response = r#"{"error":"something went wrong"}"#;

    // Mount the mock response into the server
    Mock::given(method("POST"))
        .and(path("api/bot/game/is9Gsjun/resign"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(any())
        .respond_with(ResponseTemplate::new(400).set_body_raw(error_response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.bot_game_resign("is9Gsjun").await.unwrap();
    client.bot_game_resign("o2J3suHn").await.unwrap_err();
}

#[tokio::test]
async fn bots_online() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/bots_online.json")
        .await
        .unwrap();

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
        .with_base_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    let mut stream = client.bots_online(50).await.unwrap();
    while stream.try_next().await.unwrap().is_some() {}
}
