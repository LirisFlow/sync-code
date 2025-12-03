# sync-code

Synchronize code blocks between different files.

# Usage
`Cargo.toml`:
```toml
[build-dependencies]
sync-code = "0.1.0"
```

`build.rs`:
```rust
fn main() {
    sync_code::Builder::new()
        .add("src/target1.rs", "src/source1.rs")
        .add("src/target2.rs", "src/source2.rs")
        .sync();
}

```

See also [tests](/tests)
