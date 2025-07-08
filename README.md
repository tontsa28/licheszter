# licheszter

[![crates.io](https://img.shields.io/crates/v/licheszter.svg)](https://crates.io/crates/licheszter)
[![Dependencies](https://deps.rs/repo/github/tontsa28/licheszter/status.svg)](https://deps.rs/repo/github/tontsa28/licheszter)
[![Documentation](https://docs.rs/licheszter/badge.svg)](https://docs.rs/licheszter)
[![Apache 2.0 license](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE-APACHE)
[![MIT license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

Licheszter is a Rust library that aims to wrap the entire Lichess API.
Currently, the library is under active development and more features are being added.
The goal is to eventually be the most complete and overall #1 Lichess API wrapper written in Rust.
Whether you're looking for your first open source Rust project to contribute to or you're just generally interested in the project, check the [Contributions](#contributions) section for more information.

### WARNING
**The project is unstable to use in production until the version 1.0.0 since no guarantees about breaking changes can be made.**
**Use at your own risk and prepare to face breaking changes more or less often.**
**IF YOU COME ACROSS ANY UNEXPECTED ERRORS, PLEASE OPEN A GITHUB ISSUE DESCRIBING THE ERROR.**
**As the Lichess API evolves continuously, especially the deserialization models can become inaccurate and produce errors.**
**Such bugs will be fixed ASAP.**

### NOTE
It is forbidden to use the Board API (crate feature `board`) for projects that involve use of chess engines or other things that can be interpreted as cheating.
The feature `bot` is enabled by default to prevent accidents.
If you're not developing anything that uses external chess assistance, you can enable `board` feature if the Bot API is not suitable for your use case.
You can also choose to use neither, if you simply don't need that functionality, by disabling default features.
In this case, do bear in mind that you may need to opt in to other, normally default features manually as well.
This project and its developers are NOT responsible for any account bans that may occur from the misuse of the Board API.

## Examples
Here is an example of creating an authenticated instance of `Licheszter` and using it:
```rust,no_run
use licheszter::client::Licheszter;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    // Create a new Licheszter with your account token
    let client = Licheszter::builder()
        .with_authentication("lip_exampletoken")
        .build();

    // Use the client to fetch online bots, for example...
    let bots = client.bots_online(10).await.unwrap();

    // ...or open the event stream
    let events = client.connect().await.unwrap();
    while let Some(event) = events.next().await.unwrap() {
        // Do something with the event!
    }
}
```

## Features
Below is a list of supported API endpoints as of the last release:
| Category          | Supported |
| --------          | --------- |
| Account           | ✅        |
| Users             | ✅        |
| Relations         | ✅        |
| Games             | ❌        |
| TV                | ❌        |
| Puzzles           | ✅        |
| Teams             | ❌        |
| Bot               | ✅        |
| Board             | ✅        |
| Challenges        | ✅        |
| Bulk pairings     | ❌        |
| Arena tournaments | ❌        |
| Swiss tournaments | ❌        |
| Simuls            | ✅        |
| Studies           | ❌        |
| Messaging         | ✅        |
| Broadcasts        | ❌        |
| FIDE              | ❌        |
| Analysis          | ✅        |
| External engine   | ❌        |
| Opening explorer  | ✅        |
| Tablebase         | ✅        |

## Contributions
All contributions are greatly appreciated, no matter if they provide improvements to code, documentation or anything else related to the project.
Please follow [semantic commit message](https://gist.github.com/joshbuchea/6f47e86d2510bce28f8e7f42ae84c716) guidelines in your commits.

Willing to contribute but unsure where to start?
Check the repository issues to see where your help is needed the most.
If no issues are currently open, feel free to email me at miika@tontsa.fi.

For additional information, check the official [Lichess API documentation](https://lichess.org/api).
