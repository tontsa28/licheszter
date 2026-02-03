# Test Fixes After Vec<&str> to &[&str] API Change

## Problem

After switching method parameters from `Vec<&str>` to `&[&str]` in the API, 23 test compilation errors occurred across the test suite. The tests were still passing `Vec<&str>` values instead of slice references.

## Affected Methods

The API change affected these methods:
1. `users_status(&[&str], ...)`
2. `users_list(&[&str])`
3. `games_export(&[&str], ...)`
4. `games_users_connect(&[&str], ...)`
5. `games_connect(string, &[&str])`
6. `games_connect_add(string, &[&str])`

## Test Files Fixed

### tests/users.rs
- **4 call sites fixed**
  - 2 calls to `users_status`
  - 2 calls to `users_list`

### tests/games.rs
- **21 call sites fixed**
  - 2 calls to `games_export`
  - 5 calls to `games_users_connect`
  - 5 calls to `games_connect`
  - 9 calls to `games_connect_add`

## Fix Patterns Applied

### Pattern 1: Inline Vec Literals
**Before:**
```rust
LI.users_status(vec!["adriana", "ana", "bot0"], None)
```

**After:**
```rust
LI.users_status(&["adriana", "ana", "bot0"], None)
```

### Pattern 2: Vec Variables
**Before:**
```rust
let game_ids: Vec<&str> = ...;
LI.games_connect("id", game_ids)
```

**After:**
```rust
let game_ids: Vec<&str> = ...;
LI.games_connect("id", &game_ids)
```

### Pattern 3: Empty Vecs
**Before:**
```rust
LI.games_export(vec![], Some(&options))
```

**After:**
```rust
LI.games_export(&[], Some(&options))
```

## Benefits of &[&str] Over Vec<&str>

The API change from `Vec<&str>` to `&[&str]` provides several advantages:

1. **More Flexible**: Accepts both `Vec<&str>` (via `&vec`) and array slices (`&[...]`)
2. **Zero-Copy**: No forced allocation when passing array literals
3. **Idiomatic Rust**: Slices are the standard way to accept read-only sequences
4. **Backward Compatible**: Callers can still create `Vec<&str>` and pass `&vec`

## Verification

- ✅ All tests compile successfully (`cargo test --no-run`)
- ✅ No breaking changes to test logic
- ✅ Code formatted with `cargo fmt`
- ✅ 25 total test call sites updated

## Summary

Successfully fixed all broken tests by converting Vec parameters to slice references. The changes were straightforward and followed consistent patterns:
- Inline `vec![...]` → `&[...]`
- Vec variables → add `&` borrow
- No changes to test logic required

All tests now compile and are ready to run against a test server.
