use std::{error::Error, sync::LazyLock, time::Duration};

use futures_util::{StreamExt, TryStreamExt};
use licheszter::{
    client::Licheszter,
    config::games::{ExtendedGameOptions, GameOptions, GameSortOrder},
    models::{
        game::{FinalColor, Game, StreamGame},
        user::PerfType,
    },
};
use tokio::time::{sleep, timeout};

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
        "Failed to export ongoing game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_ongoing_user("Li", None).await;
    assert!(
        result.is_ok(),
        "Failed to export ongoing game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_export_ongoing_user("Adriana", Some(&options)).await;
    assert!(
        result.is_ok(),
        "Failed to export ongoing game: {:?}",
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
            "Failed to export user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export_user("Li", None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export_user("Adriana", Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export user games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let result = LI.games_export_user("NoSuchUser", None).await;
    assert!(result.is_err(), "Exporting user games did not fail");
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
    let game_ids: Vec<&str> = games.iter().map(|game| game.id.as_str()).collect();
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
    let mut result = LI.games_export(game_ids.clone(), Some(&options)).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export(game_ids.clone(), None).await.unwrap();
    while let Some(event) = result.next().await {
        assert!(
            event.is_ok(),
            "Failed to export games: {:?}",
            event.unwrap_err().source().unwrap()
        );
    }

    let mut result = LI.games_export(vec![], Some(&options)).await.unwrap();
    assert!(
        result.next().await.is_none(),
        "Exporting games did not fail: {:?}",
        result.next().await.unwrap()
    );

    let mut result = LI.games_export(vec![], None).await.unwrap();
    assert!(
        result.next().await.is_none(),
        "Exporting games did not fail: {:?}",
        result.next().await.unwrap()
    );
}

#[tokio::test]
async fn games_users_connect() {
    // Run some test cases
    let mut result = LI.games_users_connect(vec!["li", "bot0"], true).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream user games: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let mut result = LI.games_users_connect(vec!["li", "adriana"], true).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream user games: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let mut result = LI.games_users_connect(vec!["li", "bot0"], false).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream user games: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let mut result = LI.games_users_connect(vec!["li"], false).await.unwrap();
    assert!(
        result.next().await.is_none(),
        "Streaming user games did not fail: {:?}",
        result.next().await.unwrap()
    );
}

#[tokio::test]
async fn games_connect() {
    // Get some game IDs for testing
    let games: Vec<StreamGame> = LI
        .games_users_connect(vec!["li", "bot0", "adriana"], true)
        .await
        .unwrap()
        .take(3)
        .try_collect()
        .await
        .unwrap();
    let game_ids: Vec<&str> = games.iter().map(|game| game.id.as_str()).collect();

    // Run some test cases
    let mut result = LI.games_connect("randomid", game_ids).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream games: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let mut result = LI.games_connect("randomid", vec![]).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream games: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let result = LI.games_connect("", vec![]).await;
    assert!(result.is_err(), "Streaming games did not fail");
}

#[tokio::test]
async fn games_connect_add() {
    // Start a stream of games for testing
    let task = tokio::spawn(async {
        let games: Vec<StreamGame> = LI
            .games_users_connect(vec!["li", "bot0", "adriana"], true)
            .await
            .unwrap()
            .take(3)
            .try_collect()
            .await
            .unwrap();
        let game_ids: Vec<&str> = games.iter().map(|game| game.id.as_str()).collect();

        let mut stream = LI.games_connect("someid", game_ids).await.unwrap();
        while stream.next().await.is_some() {}
    });

    // Get some game IDs for testing
    let games: Vec<StreamGame> = LI
        .games_users_connect(vec!["li", "bot0", "adriana"], true)
        .await
        .unwrap()
        .take(3)
        .try_collect()
        .await
        .unwrap();
    let game_ids: Vec<&str> = games.iter().map(|game| game.id.as_str()).collect();
    sleep(Duration::from_millis(100)).await;

    // Run some test cases
    let result = LI.games_connect_add("someid", game_ids).await;
    assert!(
        result.is_ok(),
        "Failed to add game to stream: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_connect_add("someid", vec![]).await;
    assert!(
        result.is_ok(),
        "Failed to add game to stream: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_connect_add("someid", vec!["notvalid"]).await;
    assert!(
        result.is_ok(),
        "Failed to add game to stream: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_connect_add("invalid", vec![]).await;
    assert!(result.is_err(), "Adding game to stream did not fail: {:?}", result.unwrap());

    task.abort();
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

#[tokio::test]
async fn games_moves_connect() {
    // Get some game IDs for testing
    let games = LI.games_ongoing(2).await.unwrap();
    let game_ids: Vec<&str> = games.iter().map(|game| game.game_id.as_str()).collect();

    // Run some test cases
    let mut result = LI.games_moves_connect(game_ids[0]).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream moves of a game: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let mut result = LI.games_moves_connect(game_ids[1]).await.unwrap();
    timeout(Duration::from_secs(1), async {
        while let Some(event) = result.next().await {
            assert!(
                event.is_ok(),
                "Failed to stream moves of a game: {:?}",
                event.unwrap_err().source().unwrap()
            );
        }
    })
    .await
    .unwrap_err();

    let result = LI.games_moves_connect("notvalid").await;
    assert!(result.is_err(), "Streaming moves of a game did not fail");
}

#[tokio::test]
async fn games_import_one() {
    // Use a hardcoded PGN for testing
    let pgn = r#"[Event "Li Arena"]
    [Site "http://localhost:8080/vj7imIJW"]
    [Date "2025.12.27"]
    [Round "-"]
    [White "Bot0"]
    [Black "Li"]
    [Result "0-1"]
    [GameId "vj7imIJW"]
    [UTCDate "2025.12.27"]
    [UTCTime "11:21:46"]
    [WhiteElo "1500"]
    [BlackElo "1774"]
    [WhiteRatingDiff "-119"]
    [BlackRatingDiff "+1"]
    [WhiteTitle "BOT"]
    [Variant "Standard"]
    [TimeControl "120+0"]
    [ECO "A00"]
    [Opening "Barnes Opening: Fool's Mate"]
    [Termination "Normal"]
    [Annotator "localhost:8080"]

    1. f3 e5 2. g4 Qh4# { A00 Barnes Opening: Fool's Mate } { Black wins by checkmate. } 0-1"#;

    // Run some test cases
    let result = LI.games_import_one(pgn).await;
    assert!(
        result.is_ok(),
        "Failed to import game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = BOT0.games_import_one(pgn).await;
    assert!(
        result.is_ok(),
        "Failed to import game: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.games_import_one("").await;
    assert!(result.is_err(), "Importing game did not fail: {:?}", result.unwrap());
}
