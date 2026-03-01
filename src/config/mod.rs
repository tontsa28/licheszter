#[cfg(feature = "board")]
pub mod board;

#[cfg(feature = "challenges")]
pub mod challenges;

#[cfg(feature = "games")]
pub mod games;

#[cfg(feature = "openings")]
pub mod openings;

#[cfg(feature = "pairings")]
pub mod pairings;

#[cfg(feature = "puzzles")]
pub mod puzzles;

#[cfg(feature = "tv")]
pub mod tv;

#[cfg(feature = "users")]
pub mod users;

#[cfg(any(feature = "challenges", feature = "pairings"))]
pub(super) fn set_clock(clock_limit: u16, clock_increment: u8) -> (u16, u8) {
    let limit = match clock_limit {
        0 | 15 | 30 | 45 | 60 | 90 => clock_limit,
        x if x % 60 == 0 && x <= 10800 => clock_limit,
        _ => 0,
    };
    let increment = clock_increment.min(180);
    (limit, increment)
}
