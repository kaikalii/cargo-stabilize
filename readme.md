### Description

[View on crates.io](https://crates.io/crates/cargo-stabilize)

This is simple tool which replaces all version wildcards (`*`) in your `Cargo.toml` with the newest version of the crate. It can also upgrade dependencies that already have versions.

### Installation

To install, simply run:
```
cargo install cargo-stabilize
```

### Usage

There are two primary ways to use it. **Be warned: Using this tool will reformat your entire `Cargo.toml` and delete any comments therein.** No non-comment information will be lost.

* `cargo stabilize` will replace all `*` dependency versions with the newest version of that crate.
* `cargo stabilize --upgrade` will do the same thing, as well as upgrading the versions of all dependencies whose versions are not the newest.
