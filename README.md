# Colourful-Logger

The Colourful-Logger is a simple yet effective logging utility designed to enhance the readability of log messages by incorporating vibrant colors.

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
use colourful_logger::Logger as Logger;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOGGER: Logger = Logger::new();
}

fn main(): {
    LOGGER.info("This is a message!", "Tag");
}
```

or you can use it inside of functions
```rust
use colourful_logger::Logger as Logger;

fn main(): {
    let logger = Logger::new();

    logger.info("Message", "Tag");
}
```

## Bug Reports | Features

If there are any bugs, or features you'd like to implement into the logger, feel free to create a pr request and it'll be looked into.