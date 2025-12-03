//! Synchronize code blocks between different files.
//!
//! # Usage
//! `Cargo.toml`:
//! ```toml
//! [build-dependencies]
//! sync-code = "0.1.0"
//! ```
//!
//! `build.rs`:
//! ```rust
//! fn main() {
//!     sync_code::Builder::new()
//!         .add("src/target1.rs", "src/source1.rs")
//!         .add("src/target2.rs", "src/source2.rs")
//!         .sync();
//! }
//!
//! ```

mod sync;

use std::path::Path;
use sync::Sync;

pub struct Builder {
    table: Vec<Sync>,
}

impl Builder {
    pub fn new() -> Self {
        Builder { table: Vec::new() }
    }

    pub fn add<P: AsRef<Path>>(mut self, file: P, dep_file: P) -> Self {
        println!("cargo:rerun-if-changed={}", dep_file.as_ref().display());
        self.table.push(Sync::new(
            file.as_ref().to_path_buf(),
            dep_file.as_ref().to_path_buf(),
        ));
        self
    }

    pub fn sync(&mut self) {
        println!("Start syncing code ... {}", self.table.len());
        for sync in &self.table {
            sync.sync();
        }
    }
}
