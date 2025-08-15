use async_trait::async_trait;
use fluxus::sinks::{ConsoleFormatter, DefaultFormatter, Sink};
use fluxus::utils::models::{Record, StreamError, StreamResult};
use std::marker::PhantomData;

/// A demo sink implementation for demonstration purposes
#[derive(Default, Clone)]
pub struct DemoSink<T, F = DefaultFormatter> {
    formatter: F,
    config: String,
    target: String,
    _phantom: PhantomData<T>,
}

impl<T> DemoSink<T, DefaultFormatter> {
    /// Create a new demo sink with default formatter
    pub fn new(config: String, target: String) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            target,
            formatter: DefaultFormatter,
            _phantom: PhantomData,
        })
    }
}

impl<T, F> DemoSink<T, F> {
    /// Create a new demo sink with custom formatter
    pub fn with_formatter(formatter: F) -> Self {
        Self {
            config: String::new(),
            target: String::new(),
            formatter,
            _phantom: PhantomData,
        }
    }

    /// Set the configuration for the demo sink
    pub fn set_config(&mut self, config: String) {
        self.config = config;
    }

    /// Set the target for the demo sink
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    pub fn is_initialized(&self) -> bool {
        !self.config.is_empty() && !self.target.is_empty()
    }
}

#[async_trait]
impl<T, F> Sink<T> for DemoSink<T, F>
where
    T: Send,
    F: ConsoleFormatter<T> + Send + Sync,
{
    async fn init(&mut self) -> StreamResult<()> {
        Ok(())
    }

    async fn write(&mut self, record: Record<T>) -> StreamResult<()> {
        if !self.is_initialized() {
            return Err(StreamError::Config("Demo sink not initialized".to_string()));
        }

        let message = self.formatter.format(&record);
        tracing::info!(
            "Processing record with config: {}, target: {}, message: {}",
            self.config,
            self.target,
            message
        );
        Ok(())
    }

    async fn flush(&mut self) -> StreamResult<()> {
        Ok(())
    }

    async fn close(&mut self) -> StreamResult<()> {
        Ok(())
    }
}
