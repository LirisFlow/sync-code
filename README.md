# sync-code
[![Crates.io](https://img.shields.io/crates/v/sync-code.svg)](https://crates.io/crates/sync-code)
[![Docs.rs](https://docs.rs/sync-code/badge.svg)](https://docs.rs/sync-code)
[![CI](https://github.com/mcu-rust/sync-code/workflows/CI/badge.svg)](https://github.com/mcu-rust/sync-code/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE)
[![Downloads](https://img.shields.io/crates/d/sync-code.svg)](https://crates.io/crates/sync-code)

**[sync-code](https://crates.io/crates/sync-code)** synchronizes code blocks across different files, providing a clear alternative to macros in certain scenarios.  
When duplication is limited but the design is complex (for example, involving generics), macros often reduce readability and make maintenance harder.  
By using **[sync-code](https://crates.io/crates/sync-code)** instead, the code remains straightforward and maintainable, while development tools such as *Go-to-definition* and intelligent auto-completion continue to work as expected.

# Usage
Run command:
```shell
cargo add --build sync-code
```

See [crate](https://crates.io/crates/sync-code).

`build.rs`:
```rust
fn main() {
    sync_code::Builder::new()
        .add("src/target1.rs", "src/source1.rs")
        .add("src/target2.rs", "src/source2.rs")
        .sync();
}

```

`your_code.rs`:
```rust
// $sync block_name

fn code_you_want_to_sync() {
}

// $sync end
```

See also [tests](/tests/test.rs).

## 🔖 Keywords
sync-code · rust · code generation · macro replacement · generics · readability · maintainability · developer tools 


