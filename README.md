# Microlog

Dead simple log subscriber, coming in at only `38` meaningful lines of code.

# Docs

Run `cargo doc --open` to view the full documentation.

# Example

```rs
microlog::init(microlog::LevelFilter::Trace);

log::trace!("Trace test");
log::debug!("Debug test");
log::info!("Info test");
log::warn!("Info test");
log::error!("Info test");
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
