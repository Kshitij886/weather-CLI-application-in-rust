# weather-cli

A small Rust command-line application that fetches and displays current weather for a given city using the API Ninjas weather API.

## Prerequisites
- Rust (stable) and Cargo installed
- An API key from https://api.api-ninjas.com

## Quick Start

1. Build the project:

```bash
cargo build --release
```

2. Run the app:

```bash
cargo run 
```

The program is interactive and will prompt you to enter a city name.

## API Key
The example hardcodes an API key in `src/main.rs`. Replace the value of `api_key` with your personal API key from API Ninjas before running, or modify the code to load the key from an environment variable.

## Notes
- Uses `reqwest`, `serde`, `tokio`, and `colored`.
- The JSON response is deserialized into `types::weather_types::WeatherResponse`.

