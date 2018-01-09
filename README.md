simple_write_logger
===

simple logging library for rust

## Usage

```toml
[dependencies]
log = "0.4"

[dependencies.simple_write_logger]
git = "https://github.com/bnuts/simple_write_logger.rs.git"
```

```rust
#[macro_use]
extern crate log;
extern crate simple_write_logger;

fn main() {
    use std::fs::File;
    use std::io::stdout;

    let file = File::create("log.txt").unwrap();
    let writers = vec![
        simple_write_logger::Writer(Box::new(stdout())),
        simple_write_logger::Writer(Box::new(file)),
    ];
    simple_write_logger::init(writers, log::Level::Debug).unwrap();

    debug!("DEBUG");
}
```
