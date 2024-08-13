# Clap All-Gen

[![CI](https://github.com/wiktor-k/clap_allgen/actions/workflows/rust.yml/badge.svg)](https://github.com/wiktor-k/clap_allgen/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/clap_allgen)](https://crates.io/crates/clap_allgen)

The complete clap generation utility to give your command-line application users a more polished experience right out of the box.

This single crate integrates all other shell-specific crates to generate *all the things* at once.

## Examples

To create all shell completions use the following command:

```rust
use clap_allgen::render_shell_completions;

#[derive(Debug, clap::Parser)]
enum Commands {
    First,
    Second,
    Third,
}

render_shell_completions::<Commands>("/tmp/shell-completions").expect("generation to work");
```

To generate man pages for your commands use:

```rust
use clap_allgen::render_manpages;

#[derive(Debug, clap::Parser)]
enum Commands {
    First,
    Second,
    Third,
}

render_manpages::<Commands>("/tmp/man-pages").expect("generation to work");
```

## License

This project is licensed under either of:

  - [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0),
  - [MIT license](https://opensource.org/licenses/MIT).

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
