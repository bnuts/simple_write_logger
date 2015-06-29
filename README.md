simple_write_logger
===

simple logging library for rust

## Usage

```toml
[dependencies]
log = "0.3"

[dependencies.simple_write_logger]
git = "https://github.com/bnuts/simple_write_logger.git"
```

```rust
#[macro_use] extern crate log;
extern crate simple_write_logger;

use std::fs::File;
use std::io::stdout;

fn main() {
    let file = File::create("log.txt");
    let writers = vec![
        simple_write_logger::Writer(Box::new(stdout())),
        simple_write_logger::Writer(Box::new(file.unwrap())),
    ];
    simple_write_logger::init(writers).unwrap();

    debug!("DEBUG");
}
```
