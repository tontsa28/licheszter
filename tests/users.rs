#![cfg(feature = "users")]

use std::{error::Error, sync::LazyLock};

use licheszter::{client::Licheszter, config::users::UserStatusOptions, models::user::PerfType};

// Connect to test clients
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
        .unwrap()
        .build()
});

static DEFAULT: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .build()
});

#[tokio::test]
async fn users_status() {
    // Create options for testing
    let options = UserStatusOptions::new()
        .signal(true)
        .game_ids(false)
        .game_metas(true);

    // Run some test cases
    let result = LI.users().status(&["adriana", "ana", "bot0"], None).await;
    assert!(
        result.is_ok(),
        "Failed to get user statuses: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users()
        .status(&["adriana", "ana", "bot0"], Some(&options))
        .await;
    assert!(
        result.is_ok(),
        "Failed to get user statuses: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_top10() {
    // Run some test cases
    let result = LI.users().top10().await;
    assert!(
        result.is_ok(),
        "Failed to get top 10 lists: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().top10().await;
    assert!(
        result.is_ok(),
        "Failed to get top 10 lists: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_leaderboard() {
    // Run some test cases
    let result = LI.users().leaderboard(20, PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().leaderboard(5, PerfType::Chess960).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().leaderboard(10, PerfType::Puzzle).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().leaderboard(20, PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_profile() {
    // Run some test cases
    let result = LI.users().profile("Li", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().profile("Adriana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().profile("Ana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().profile("Bot0", false).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().profile("Bot0", false).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().profile("NoSuchUser", true).await;
    assert!(result.is_err(), "Getting user profile did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn users_rating_history() {
    // Run some test cases
    let result = LI.users().rating_history("Li").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().rating_history("Adriana").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().rating_history("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().rating_history("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Getting user rating history did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn users_performance() {
    // Run some test cases
    let result = LI.users().performance("Li", PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get user performance statistics: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().performance("Bot0", PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get user performance statistics: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().performance("Ana", PerfType::Rapid).await;
    assert!(
        result.is_ok(),
        "Failed to get user performance statistics: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().performance("NoSuchUser", PerfType::Bullet).await;
    assert!(
        result.is_err(),
        "Getting user performance statistics did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn users_activity() {
    // Run some test cases
    let result = LI.users().activity("Li").await;
    assert!(
        result.is_ok(),
        "Failed to get user activity feed: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().activity("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to get user activity feed: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().activity("Ana").await;
    assert!(
        result.is_ok(),
        "Failed to get user activity feed: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().activity("NoSuchUser").await;
    assert!(
        result.as_ref().is_ok_and(|vec| vec.is_empty()),
        "Getting user activity feed did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn users_list() {
    // Run some test cases
    let result = LI.users().list(&["Ana", "Adriana", "Bot0"]).await;
    assert!(
        result.is_ok(),
        "Failed to get list of users: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().list(&["Ana", "Adriana", "Bot0"]).await;
    assert!(
        result.is_ok(),
        "Failed to get list of users: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_streamers_live() {
    // Run some test cases
    let result = LI.users().streamers_live().await;
    assert!(
        result.is_ok(),
        "Failed to get live streamers: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().streamers_live().await;
    assert!(
        result.is_ok(),
        "Failed to get live streamers: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_crosstable() {
    // Run some test cases
    let result = LI.users().crosstable("Li", "Adriana", false).await;
    assert!(
        result.is_ok(),
        "Failed to get crosstable between users: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().crosstable("Li", "Adriana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get crosstable between users: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().crosstable("Li", "Adriana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get crosstable between users: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().crosstable("NoSuchUser1", "NoSuchUser2", true).await;
    assert!(
        result.is_ok(),
        "Failed to get crosstable between users: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_autocomplete() {
    // Run some test cases
    let result = LI.users().autocomplete("bot", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().autocomplete("bot", true).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().autocomplete("NoSuch", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().autocomplete("bot", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_autocomplete_details() {
    // Run some test cases
    let result = LI.users().autocomplete_details("bot", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().autocomplete_details("bot", true).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().autocomplete_details("NoSuch", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = DEFAULT.users().autocomplete_details("bot", false).await;
    assert!(
        result.is_ok(),
        "Failed to get autocompletion for name: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_notes_write() {
    // Run some test cases
    let result = LI.users().notes_write("Li", "This is a private test note").await;
    assert!(
        result.is_ok(),
        "Failed to write to private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users()
        .notes_write("Adriana", "This is a private test note")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users()
        .notes_write("NoSuchUser", "This is a private test note")
        .await;
    assert!(result.is_err(), "Writing to private notes did not fail: {:?}", result.unwrap());

    let result = DEFAULT
        .users()
        .notes_write("Bot0", "This is a private test note")
        .await;
    assert!(result.is_err(), "Writing to private notes did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn users_notes_read() {
    // Run some test cases
    let result = LI.users().notes_read("Li").await;
    assert!(
        result.is_ok(),
        "Failed to read private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().notes_read("Adriana").await;
    assert!(
        result.is_ok(),
        "Failed to read private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users().notes_read("NoSuchUser").await;
    assert!(result.is_err(), "Reading private notes did not fail: {:?}", result.unwrap());

    let result = DEFAULT.users().notes_read("Bot0").await;
    assert!(result.is_err(), "Reading private notes did not fail: {:?}", result.unwrap());
}
