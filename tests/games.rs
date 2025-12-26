use std::{error::Error, sync::LazyLock};

use futures_util::StreamExt;
use licheszter::{
    client::Licheszter,
    config::games::{ExtendedGameOptions, GameOptions, GameSortOrder},
    models::{
        game::{FinalColor, Game},
        user::PerfType,
    },
};

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .build()
});

static BOT0: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_bot0")
        .build()
});

#[tokio::test]
async fn games_export_one() {
    // Create options and games for testing
    let games_options = ExtendedGameOptions::new().max(2);
    let games: Vec<Game> = LI
        .games_export_user("Li", Some(&games_options))
        .await
        .unwrap()
        .map(|event| event.unwrap())
        .collect()
        .await;
    let options = GameOptions::new()
        .moves(true)
        .tags(true)
        .clocks(true)
        .evals(true)
        .accuracy(true)
        .opening(true)
        .division(true)
        .literate(true);

    // Run some test cases
    let result = LI.games_export_one(&games[0].id, Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to export game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_one(&games[1].id, None).await;
    assert!(
        result.is_ok(),
        "Failed to export game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_one("notvalid", None).await;
    assert!(result.is_err(), "Exporting one game did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn games_export_ongoing_user() {
    // Create options for testing
    let options = GameOptions::new()
        .moves(true)
        .tags(true)
        .clocks(true)
        .evals(true)
        .accuracy(true)
        .opening(true)
        .division(true)
        .literate(true);

    // Run some test cases
    let result = LI.games_export_ongoing_user("Li", Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_ongoing_user("Li", None).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_ongoing_user("Adriana", Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_ongoing_user("NoSuchUser", None).await;
    assert!(result.is_err(), "Exporting ongoing game did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn games_export_user() {
    // Create options for testing
    let options = ExtendedGameOptions::new()
        .max(10)
        .rated(false)
        .perf_type(vec![PerfType::Bullet, PerfType::Blitz, PerfType::Rapid])
        .color(FinalColor::White)
        .analysed(false)
        .moves(true)
        .tags(true)
        .clocks(true)
        .evals(true)
        .accuracy(true)
        .opening(true)
        .division(true)
        .ongoing(true)
        .finished(true)
        .literate(true)
        .last_fen(true)
        .with_bookmarked(true)
        .sort(GameSortOrder::DateDesc);

    // Run some test cases
    let mut result = LI.games_export_user("Li", Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export_user("Li", None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export_user("Adriana", Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = LI.games_export_user("NoSuchUser", None).await;
    assert!(result.is_err(), "Exporting games did not fail");
}

#[tokio::test]
async fn games_export() {
    // Create options and games for testing
    let games_options = ExtendedGameOptions::new().max(10);
    let games: Vec<Game> = LI
        .games_export_user("Li", Some(&games_options))
        .await
        .unwrap()
        .map(|event| event.unwrap())
        .collect()
        .await;
    let ids: Vec<&str> = games.iter().map(|game| game.id.as_str()).collect();
    let options = GameOptions::new()
        .moves(true)
        .tags(true)
        .clocks(true)
        .evals(true)
        .accuracy(true)
        .opening(true)
        .division(true)
        .literate(true);

    // Run some test cases
    let mut result = LI.games_export(ids.clone(), Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get ongoing game: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export(ids.clone(), None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to get ongoing game: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export(vec![], Some(&options)).await.unwrap();
    assert!(
        result.next().await.is_none(),
        "Failed to export games: {:?}",
        result.next().await.unwrap().unwrap_err().source().unwrap()
    );

    let mut result = LI.games_export(vec![], None).await.unwrap();
    assert!(
        result.next().await.is_none(),
        "Failed to export games: {:?}",
        result.next().await.unwrap().unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn games_ongoing() {
    // Run some test cases
    let result = LI.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.games_ongoing(10).await;
    assert!(
        result.is_ok(),
        "Failed to get ongoing games: {:?}",
        result.unwrap_err().source().unwrap()
    );
}
