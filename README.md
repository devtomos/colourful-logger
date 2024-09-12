# Colourful-Logger

The Colourful-Logger is a simple yet effective logging utility designed to enhance the readability of log messages by incorporating vibrant colors. This tool is particularly useful for developers who need to quickly identify and differentiate between various types of log messages, such as errors, warnings, and informational messages. By using distinct colors for different log levels, the Colourful-Logger makes it easier to spot critical issues and track the flow of execution in your Rust applications. Its straightforward implementation ensures that it can be easily integrated into any project, providing an immediate visual improvement to your logging output.

## Features
- Easy to use
- Colour coded log levels
- Quick identification of log types
- Enhanced readability
- Simple integration into projects
- Immediate visual improvement

## How to use
You can use either lazy_static! to use the logger as a global variable
```rust
use colourful-logger::{Logger};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: logger = logger::new();
}```

or you can use it inside of functions
```rust

fn main(): {
    let logger = logger::new();

    logger.infoSingle("Message", "Tag");
}```


## Log Levels

There are 6 log levels in total:
- Fatal
- Error
- Warn
- Info
- Debug
- Silly

Each being more serve for their purpose inside the code.

## Bug Reports | Features

If there are any bugs, or features you'd like to implement into the logger, feel free to create a pr request and it'll be looked into.