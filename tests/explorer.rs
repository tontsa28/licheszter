use futures_util::TryStreamExt;
use licheszter::client::Licheszter;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn opening_masters() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/explorer_masters.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("masters"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_explorer_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.opening_masters(None).await.unwrap();
}

#[tokio::test]
async fn opening_lichess() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/explorer_lichess.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("lichess"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_explorer_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.opening_lichess(None).await.unwrap();
}

#[tokio::test]
async fn opening_player() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/explorer_player.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("player"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(response, "application/x-ndjson")
                .append_header("Transfer-Encoding", "chunked"),
        )
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_explorer_url(&mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    let mut stream = client.opening_player(None).await.unwrap();
    while stream.try_next().await.unwrap().is_some() {}
}
