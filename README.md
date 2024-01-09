# Clap All-Gen

The complete clap generation utility to give your command-line application users a more polished experience right out of the box.

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
