use futures_util::TryStreamExt;
use licheszter::{client::Licheszter, error::Result};

#[tokio::test]
async fn opening_masters() -> Result<()> {
    let client = Licheszter::new_unauthenticated();

    let data = client.opening_masters(None).await;
    assert!(data.is_ok() || data.is_err_and(|err| !err.is_json()));
    Ok(())
}

#[tokio::test]
async fn opening_lichess() -> Result<()> {
    let client = Licheszter::new_unauthenticated();

    let data = client.opening_lichess(None).await;
    assert!(data.is_ok() || data.is_err_and(|err| !err.is_json()));
    Ok(())
}

#[tokio::test]
async fn opening_player() -> Result<()> {
    let client = Licheszter::new_unauthenticated();

    let mut stream = client
        .opening_player(Some(&[("player", "tontsa28"), ("color", "white")]))
        .await?;
    let event = stream.try_next().await;
    assert!(event.is_ok() || event.is_err_and(|err| !err.is_json()));
    Ok(())
}
