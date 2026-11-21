# firmetic analyze

**Status:** Work in progress.

`firmetic-analyze` will inspect firmware artifacts and emit evidence, findings,
and claims. The intended command is a `firmetic-<verb>` subcommand invoked by the
separate `firmetic` dispatcher.

## Development

```sh
cargo build --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo deny check
```

Direct dependencies are pinned to exact versions (`=x.y.z`), `Cargo.lock` is
committed, and builds use `--locked`.

## License

Licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion is offered under the terms of both licenses, without additional
terms or conditions.
