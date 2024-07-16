use futures_util::TryStreamExt;
use licheszter::client::Licheszter;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn events_stream() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/stream_event.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("api/stream/event"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(response, "application/x-ndjson")
                .append_header("Transfer-Encoding", "chunked"),
        )
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    let mut stream = client.events_stream().await.unwrap();
    while stream.try_next().await.unwrap().is_some() {}
}

#[tokio::test]
async fn games_ongoing() {
    // Start the mock server & get the response from a file
    let mock_server = MockServer::start().await;
    let response = tokio::fs::read_to_string("tests/responses/account_playing.json")
        .await
        .unwrap();

    // Mount the mock response into the server
    Mock::given(method("GET"))
        .and(path("api/account/playing"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response, "application/json"))
        .mount(&mock_server)
        .await;

    // Create a new instance of Licheszter
    let client = Licheszter::builder()
        .with_base_url(mock_server.uri())
        .unwrap()
        .build();

    // Call the mock
    client.games_ongoing(1).await.unwrap();
}
