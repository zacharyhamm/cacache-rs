# cacache [![CI](https://github.com/zkat/cacache-rs/workflows/CI/badge.svg)](https://github.com/zkat/cacache-rs/actions) [![crates.io](https://img.shields.io/crates/v/cacache.svg)](https://crates.io/crates/cacache)

A high-performance, concurrent, content-addressable disk cache, optimized for async APIs.

## Example

```rust
use cacache;
use async_attributes;

#[async_attributes::main]
async fn main() -> Result<(), cacache::Error> {
    let dir = String::from("./my-cache");

    // Write some data!
    cacache::write(&dir, "key", b"my-async-data").await?;

    // Get the data back!
    let data = cacache::read(&dir, "key").await?;
    assert_eq!(data, b"my-async-data");

    // Clean up the data!
    cacache::rm::all(&dir).await?;
}
```

## Install

Using [`cargo-edit`](https://crates.io/crates/cargo-edit)

`$ cargo add cacache`

Minimum supported Rust version is `1.43.0`.

## Documentation

- [API Docs](https://docs.rs/cacache)

## Features

- First-class async support, using either
  [`async-std`](https://crates.io/crates/async-std) or
  [`tokio`](https://crates.io/crates/tokio) as its runtime. Sync APIs are
  available but secondary. You can also use sync APIs only and remove the
  async runtime dependency.
- `std::fs`-style API
- Extraction by key or by content address (shasum, etc)
- [Subresource Integrity](#integrity) web standard support
- Multi-hash support - safely host sha1, sha512, etc, in a single cache
- Automatic content deduplication
- Atomic content writes even for large data
- Fault tolerance (immune to corruption, partial writes, process races, etc)
- Consistency guarantees on read and write (full data verification)
- Lockless, high-concurrency cache access
- Really helpful, contextual error messages
- Large file support
- Pretty darn fast
- Arbitrary metadata storage
- Cross-platform: Windows and case-(in)sensitive filesystem support
- [`miette`](https://crates.io/crates/miette) integration for detailed, helpful error reporting.
- Punches nazis

`async-std` is the default async runtime. To use `tokio` instead, turn off
default features and enable the `tokio-runtime` feature, like this:

```toml
[dependencies]
cacache = { version = "*", default-features = false, features = ["tokio-runtime", "mmap"] }
```

You can also remove async APIs altogether, including removing async runtime
dependency:

```toml
[dependencies]
cacache = { version = "*", default-features = false, features = ["mmap"] }
```

Experimental support for symlinking to existing files is provided via the
"link_to" feature.

## Contributing

The cacache team enthusiastically welcomes contributions and project
participation! There's a bunch of things you can do if you want to contribute!
The [Contributor Guide](CONTRIBUTING.md) has all the information you need for
everything from reporting bugs to contributing entire new features. Please
don't hesitate to jump in if you'd like to, or even ask us questions if
something isn't clear.

All participants and maintainers in this project are expected to follow [Code
of Conduct](CODE_OF_CONDUCT.md), and just generally be excellent to each
other.

Happy hacking!

## MSRV

The Minimum Supported Rust Version for cacache is `1.67.0`. Any changes to the
MSRV will be considered breaking changes.

## License

This project is licensed under [the Apache-2.0 License](LICENSE.md).
