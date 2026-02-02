# API Design Guidelines

This document outlines the design principles and patterns used in the licheszter API wrapper.

## Core Design Principles

### 1. Ergonomics and Zero-Copy

The API is designed to minimize unnecessary allocations and copies:

- **Slice parameters**: Functions accepting multiple strings use `&[&str]` instead of `Vec<&str>`, allowing callers to pass both vectors and slices without forcing allocations.
  
  ```rust
  // Good: Accepts slices without forcing allocation
  pub async fn users_status(&self, user_ids: &[&str]) -> Result<Vec<RealtimeUser>>
  
  // Usage works with both:
  client.users_status(&["user1", "user2"]).await?;
  let vec = vec!["user1", "user2"];
  client.users_status(&vec).await?;
  ```

- **Borrowed strings**: All string parameters use `&str` to avoid unnecessary `String` allocations.

### 2. Consistent Method Naming

All API methods follow a hierarchical naming convention:

```
{category}_{action}_{optional_modifier}
```

Examples:
- `account_profile()` - Get account profile
- `users_list()` - List users
- `bot_game_connect()` - Connect to bot game stream
- `challenge_create()` - Create a challenge
- `games_export_one()` - Export a single game

### 3. Streaming vs. Single Response

Methods that return streams use clear naming:
- `_connect` suffix indicates a streaming endpoint that keeps a connection open
- Regular methods return single responses

```rust
// Single response
pub async fn challenge_create(&self, username: &str) -> Result<Challenge>

// Streaming response  
pub async fn challenge_create_connect(&self, username: &str) 
    -> Result<Pin<Box<dyn Stream<Item = Result<ChallengeComplete>> + Send>>>
```

### 4. Error Handling

All API methods:
- Return `Result<T>` where `T` is the success type
- Include `# Errors` documentation sections describing failure conditions
- Use a unified error type that preserves error sources

```rust
/// Get account profile.
///
/// # Errors
/// Returns an error if the API request fails or the response cannot be deserialized.
pub async fn account_profile(&self) -> Result<User>
```

### 5. Options Pattern

Complex parameters use dedicated option structs:

```rust
let options = ChallengeOptions::new()
    .clock(Clock::new(600, 0))
    .rated(true);

client.challenge_create("opponent", Some(&options)).await?;
```

Options are always passed as `Option<&T>` to avoid unnecessary clones.

### 6. Internal Helpers

The client provides internal helper methods to reduce code duplication:

- `to_model<T>()` - Deserialize response to a model type
- `to_stream<T>()` - Create a stream of deserialized items
- `execute()` - Execute a request that returns unit `()`
- `to_string()` - Get raw string response

These helpers centralize error handling and response processing.

### 7. Builder Pattern

The `Licheszter` client uses a builder for configuration:

```rust
let client = Licheszter::builder()
    .with_authentication("lip_token")
    .with_base_url("https://lichess.org")?
    .build();
```

## Best Practices for Contributors

When adding new API endpoints:

1. **Use `&[&str]` for array parameters**, not `Vec<&str>`
2. **Use `&str` for single string parameters**, not `String`
3. **Use `Option<&T>` for optional parameters**, not `Option<T>`
4. **Use `execute()` helper** for methods returning `Result<()>`
5. **Document errors** with a `# Errors` section
6. **Follow naming conventions** consistently
7. **Use appropriate return types**:
   - `Result<Model>` for single items
   - `Result<Vec<Model>>` for collections
   - `Result<Pin<Box<dyn Stream<...>>>>` for streams
   - `Result<()>` for operations with no return value

## Example Method Implementation

```rust
/// Get the status of one or more users at the same time.
/// Works with up to 100 users.
///
/// # Errors
/// Returns an error if the API request fails or the response cannot be deserialized.
pub async fn users_status(
    &self,
    user_ids: &[&str],
    options: Option<&UserStatusOptions>,
) -> Result<Vec<RealtimeUser>> {
    let mut url = self.req_url(UrlBase::Lichess, "api/users/status");

    if let Some(options) = options {
        let encoded = comma_serde_urlencoded::to_string(options)?;
        url.set_query(Some(&encoded));
    }

    let builder = self.client.get(url).query(&[("ids", user_ids.join(","))]);
    self.to_model::<Vec<RealtimeUser>>(builder).await
}
```

## Migration Guide for Breaking Changes

If updating existing methods to follow these guidelines:

1. Change `Vec<&str>` to `&[&str]` - **Non-breaking**: `vec![]` literals work with both
2. Add error documentation - **Non-breaking**: Pure addition
3. Replace `to_model::<OkResponse>().await?; Ok(())` with `execute().await` - **Non-breaking**: Behavioral equivalent

These changes improve ergonomics without breaking existing code.
