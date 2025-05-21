# Weather Alert

A Rust application for weather alerts and notifications.

## Features

- Weather data fetching
- Alert system
- Command-line interface

## Setup

1. Make sure you have Rust installed
2. Clone this repository
3. Run `cargo build` to build the project
4. Run `cargo run` to start the application

## Dependencies

See `Cargo.toml` for a complete list of dependencies.

## License

MIT

## run
```bash
docker run -e ALERT_URL=http://apprise.xxxxx/notify/test -e LOCATION=1,1 --rm sucicada/weather-alert:latest
```
