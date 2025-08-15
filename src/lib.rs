//! Fluxus Sink Demo
//!
//! A demo sink component for the Fluxus stream processing framework that demonstrates
//! how to create a custom sink implementation. This template provides a foundation
//! for building your own sink components.
//!
//! # Features
//!
//! - Basic sink implementation structure
//! - Example configuration handling
//! - Asynchronous message processing
//! - Error handling and retry mechanisms
//!
//! # Example
//!
//! ```rust,no_run
//! use fluxus_sink_demo::DemoSink;
//! use fluxus::sinks::Sink;
//! use fluxus::utils::models::Record;
//! use std::time::SystemTime;
//!
//! fn current_time() -> i64 {
//!     SystemTime::now()
//!         .duration_since(SystemTime::UNIX_EPOCH)
//!         .unwrap()
//!         .as_secs() as i64
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut sink = DemoSink::new(
//!         "demo-config".to_string(),
//!         "demo-target".to_string(),
//!     )?;
//!
//!     let record = Record {
//!         data: "Hello from Fluxus!".to_string(),
//!         timestamp: current_time(),
//!     };
//!
//!     sink.write(record).await?;
//!
//!     Ok(())
//! }
//! ```

mod demo;
pub use demo::DemoSink;
