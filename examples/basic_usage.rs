use fluxus::sinks::{ConsoleFormatter, Sink};
use fluxus::utils::models::Record;
use fluxus_sink_demo::DemoSink;
use std::time::SystemTime;

// Custom formatter implementation
struct CustomFormatter;

impl<T> ConsoleFormatter<T> for CustomFormatter {
    fn format(&self, record: &Record<T>) -> String {
        format!("[Custom Format] Time: {}", record.timestamp)
    }
}

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    // Example 1: Basic usage with default formatter
    let mut basic_sink = DemoSink::new("basic-config".to_string(), "basic-target".to_string())?;

    let record = Record {
        data: "Hello from basic sink!".to_string(),
        timestamp: current_time(),
    };

    basic_sink.write(record).await?;

    // Example 2: Using custom formatter
    let mut custom_sink = DemoSink::with_formatter(CustomFormatter);
    custom_sink.set_config("custom-config".to_string());
    custom_sink.set_target("custom-target".to_string());

    let record = Record {
        data: "Hello from custom sink!".to_string(),
        timestamp: current_time(),
    };

    custom_sink.write(record).await?;

    Ok(())
}
