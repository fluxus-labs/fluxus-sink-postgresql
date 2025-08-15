# Fluxus Sink Demo

A demo sink component for the Fluxus stream processing framework, demonstrating how to create a custom sink implementation. This template provides a foundation for building your own sink components.

## Features

- Basic sink implementation structure
- Example configuration handling
- Asynchronous message processing
- Error handling and retry mechanisms
- Easy integration with Fluxus framework

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fluxus-sink-demo = "0.1"
```

## Usage

### Basic Example

```rust
use fluxus_sink_demo::DemoSink;
use fluxus::sinks::Sink;
use fluxus::utils::models::Record;
use std::time::SystemTime;

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the Demo sink
    let mut sink = DemoSink::new(
        "demo-config".to_string(),
        "demo-target".to_string(),
        None, // Optional configuration
    )?;

    let record = Record {
        data: "Hello from Fluxus!".to_string(),
        timestamp: current_time(),
    };

    // Process the record
    sink.write(record).await?;

    Ok()
}
```

### Using with Custom Configuration

```rust
let mut sink = DemoSink::new(
    "demo-config".to_string(),
    "demo-target".to_string(),
    Some("custom-config-value".to_string()),
)?;
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
