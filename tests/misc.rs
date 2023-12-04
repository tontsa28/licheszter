use futures_util::TryStreamExt;
use licheszter::{client::Licheszter, error::Result};

#[tokio::test]
async fn stream_events() -> Result<()> {
    let token = std::env::var("TOKEN").unwrap();
    let client = Licheszter::builder().with_authentication(token).build();

    let mut stream = client.stream_events().await?;
    assert!(stream.try_next().await?.is_some());
    Ok(())
}

#[tokio::test]
async fn get_ongoing_games() -> Result<()> {
    let token = std::env::var("TOKEN").unwrap();
    let client = Licheszter::builder().with_authentication(token).build();

    client.get_ongoing_games(10).await?;
    Ok(())
}
