use fluxus::sinks::{ConsoleFormatter, Sink};
use fluxus::utils::models::Record;
use fluxus_sink_demo::DemoSink;
use std::time::SystemTime;

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

#[tokio::test]
async fn test_demo_sink_initialization() {
    let sink =
        DemoSink::<String>::new("test-config".to_string(), "test-target".to_string()).unwrap();

    assert!(sink.is_initialized());
}

#[tokio::test]
async fn test_demo_sink_with_custom_formatter() {
    let mut sink = DemoSink::<String, CustomFormatter>::with_formatter(CustomFormatter);
    assert!(!sink.is_initialized());

    sink.set_config("custom-config".to_string());
    sink.set_target("custom-target".to_string());
    assert!(sink.is_initialized());
}

#[tokio::test]
async fn test_demo_sink_write_success() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt().init();

    // Test with default formatter
    let mut basic_sink = DemoSink::new("test-config".to_string(), "test-target".to_string())?;
    let record = Record {
        data: "Test basic sink message".to_string(),
        timestamp: current_time(),
    };
    basic_sink.write(record).await?;

    // Test with custom formatter
    let mut custom_sink = DemoSink::with_formatter(CustomFormatter);
    custom_sink.set_config("custom-config".to_string());
    custom_sink.set_target("custom-target".to_string());

    let record = Record {
        data: "Test custom sink message".to_string(),
        timestamp: current_time(),
    };
    custom_sink.write(record).await?;

    Ok(())
}

#[tokio::test]
async fn test_demo_sink_write_error() {
    let mut sink = DemoSink::<String, CustomFormatter>::with_formatter(CustomFormatter);
    let record = Record {
        data: "Test message".to_string(),
        timestamp: current_time(),
    };

    let result = sink.write(record).await;
    assert!(result.is_err());
}
