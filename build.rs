use std::env;

/*
    This build logic is used to ensure that the features `bot` and `board` are not enabled at the same time.
    Enabling both is forbidden as these features partly override each other.
*/

fn main() {
    let bot = env::var("CARGO_FEATURE_BOT").is_ok();
    let board = env::var("CARGO_FEATURE_BOARD").is_ok();
    let dev = env::var("CARGO_FEATURE_DEV").is_ok();

    if bot && board && !dev {
        panic!("Features `bot` and `board` cannot be enabled at the same time.");
    }

    if dev {
        println!("cargo:warning=The `dev` feature is enabled.");
        println!("cargo:warning=The use of this feature is not recommended in production as it is only meant for the library development.");
    }
}
