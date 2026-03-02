#[cfg(any(feature = "challenges", feature = "pairings"))]
macro_rules! impl_clock_game_methods {
    ($t:ty) => {
        impl $t {
            /// Determines the clock settings for the game.
            /// Invalid clock limit values default to 0 and clock increment values over 180 default to 180.
            /// Defaults to a correspondence game.
            #[must_use]
            pub fn clock(mut self, clock_limit: u16, clock_increment: u8) -> Self {
                let (limit, increment) = super::set_clock(clock_limit, clock_increment);
                self.clock_limit = Some(limit);
                self.clock_increment = Some(increment);
                self
            }

            /// Determines the length of a correspondence game in days.
            /// Clock settings must be omitted.
            /// Defaults to unlimited.
            #[must_use]
            pub fn days(mut self, days: CorrespondenceDays) -> Self {
                self.days = Some(days as u8);
                self
            }

            /// Determines the game variant.
            /// Defaults to Standard.
            #[must_use]
            pub fn variant(mut self, variant: VariantMode) -> Self {
                self.variant = Some(variant);
                self
            }

            /// Determines a custom FEN string for the game.
            /// Requires the variant to be set as Standard, FromPosition or Chess960.
            /// Also requires the game *NOT* to be rated.
            /// Defaults to the default chess starting position.
            #[must_use]
            pub fn fen(mut self, fen: &str) -> Self {
                self.fen = Some(fen.to_string());
                self
            }
        }
    };
}

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
