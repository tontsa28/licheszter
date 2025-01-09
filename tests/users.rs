use std::{error::Error, sync::LazyLock};

use licheszter::{client::Licheszter, config::users::UserStatusOptions, models::user::PerfType};

// Connect to test accounts
static LI: LazyLock<Licheszter> = LazyLock::new(|| {
    Licheszter::builder()
        .with_base_url("http://localhost:8080")
        .unwrap()
        .with_authentication("lip_li")
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
    let result = LI.users_status(vec!["adriana", "ana", "bot0"], None).await;
    assert!(
        result.is_ok(),
        "Failed to get user statuses: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users_status(vec!["adriana", "ana", "bot0"], Some(&options))
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
    let result = LI.users_top10().await;
    assert!(
        result.is_ok(),
        "Failed to get top 10 lists: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().users_top10().await;
    assert!(
        result.is_ok(),
        "Failed to get top 10 lists: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_leaderboard() {
    // Run some test cases
    let result = LI.users_leaderboard(20, PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_leaderboard(5, PerfType::Chess960).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_leaderboard(10, PerfType::Puzzle).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().users_leaderboard(20, PerfType::Blitz).await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );
}

#[tokio::test]
async fn users_profile() {
    // Run some test cases
    let result = LI.users_profile("Li", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_profile("Adriana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_profile("Ana", true).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_profile("Bot0", false).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = Licheszter::new().users_profile("Bot0", false).await;
    assert!(
        result.is_ok(),
        "Failed to get user profile: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_profile("NoSuchUser", true).await;
    assert!(result.is_err(), "Getting user profile did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn users_rating_history() {
    // Run some test cases
    let result = LI.users_rating_history("Li").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_rating_history("Adriana").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_rating_history("Bot0").await;
    assert!(
        result.is_ok(),
        "Failed to get user rating history: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_rating_history("NoSuchUser").await;
    assert!(
        result.is_err(),
        "Getting user rating history did not fail: {:?}",
        result.unwrap()
    );
}

#[tokio::test]
async fn users_notes_write() {
    // Run some test cases
    let result = LI.users_notes_write("Li", "This is a private test note").await;
    assert!(
        result.is_ok(),
        "Failed to write to private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users_notes_write("Adriana", "This is a private test note")
        .await;
    assert!(
        result.is_ok(),
        "Failed to write to private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI
        .users_notes_write("NoSuchUser", "This is a private test note")
        .await;
    assert!(result.is_err(), "Writing to private notes did not fail: {:?}", result.unwrap());

    let result = Licheszter::new()
        .users_notes_write("Bot0", "This is a private test note")
        .await;
    assert!(result.is_err(), "Writing to private notes did not fail: {:?}", result.unwrap());
}

#[tokio::test]
async fn users_notes_read() {
    // Run some test cases
    let result = LI.users_notes_read("Li").await;
    assert!(
        result.is_ok(),
        "Failed to read private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_notes_read("Adriana").await;
    assert!(
        result.is_ok(),
        "Failed to read private notes: {:?}",
        result.unwrap_err().source().unwrap()
    );

    let result = LI.users_notes_read("NoSuchUser").await;
    assert!(result.is_err(), "Reading private notes did not fail: {:?}", result.unwrap());

    let result = Licheszter::new().users_notes_read("Bot0").await;
    assert!(result.is_err(), "Reading private notes did not fail: {:?}", result.unwrap());
}
