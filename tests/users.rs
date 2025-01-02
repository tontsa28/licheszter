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

    let result = Licheszter::new()
        .users_leaderboard(20, PerfType::Blitz)
        .await;
    assert!(
        result.is_ok(),
        "Failed to get leaderboard: {:?}",
        result.unwrap_err().source().unwrap()
    );
}
