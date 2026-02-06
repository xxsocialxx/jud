// ============================================================================
// STRUCTURED LOGGING & OBSERVABILITY
// ============================================================================

use tracing::{debug, error, info, span, warn, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    EnvFilter, Registry,
};
use std::io;

/// Initialize tracing with structured logging
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

/// Initialize tracing with file rotation
pub fn init_tracing_with_file(
    log_file: &str,
) -> Result<WorkerGuard, Box<dyn std::error::Error>> {
    use tracing_appender::rolling;
    use tracing_subscriber::prelude::*;

    let file_appender = rolling::daily(log_file, "judiw.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(guard)
}

/// Instrumentation macro for database queries
#[macro_export]
macro_rules! instrument_query {
    ($name:expr, $query:expr) => {{
        let span = span!(tracing::Level::DEBUG, "query", name = $name);
        let _enter = span.enter();
        debug!(query = %$query);
        $query
    }};
}

/// Instrumentation macro for cache operations
#[macro_export]
macro_rules! instrument_cache {
    ($op:expr, $key:expr) => {{
        let span = span!(tracing::Level::TRACE, "cache", operation = $op);
        let _enter = span.enter();
        trace!(key = %$key);
        // Perform cache operation
    }};
}

// ============================================================================
// PERFORMANCE TIMING
// ============================================================================

use std::time::{Duration, Instant};

/// Performance timer for measuring operations
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    pub fn start(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }

    pub fn finish(self) -> Duration {
        let elapsed = self.elapsed();
        debug!(
            name = %self.name,
            duration_ms = elapsed.as_secs_f64() * 1000.0,
            "Timer finished"
        );
        elapsed
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        debug!(
            name = %self.name,
            duration_ms = elapsed.as_secs_f64() * 1000.0,
            "Timer dropped"
        );
    }
}

/// Timing wrapper for async functions
pub async fn timed<T, F, E>(name: &str, f: F) -> Result<T, E>
where
    F: std::future::Future<Output = Result<T, E>>,
{
    let start = Instant::now();
    let result = f.await;
    let elapsed = start.elapsed();

    match &result {
        Ok(_) => {
            debug!(
                name = %name,
                duration_ms = elapsed.as_secs_f64() * 1000.0,
                "Operation succeeded"
            );
        }
        Err(_) => {
            error!(
                name = %name,
                duration_ms = elapsed.as_secs_f64() * 1000.0,
                "Operation failed"
            );
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let timer = Timer::start("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.finish();

        assert!(elapsed.as_millis() >= 10);
    }
}
