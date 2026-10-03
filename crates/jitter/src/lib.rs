//! ApexCode Jitter Engine
//!
//! This crate provides temporal obfuscation capabilities to simulate
//! human development timing.
//!
//! Built with ❤️ by Mohamed Alieu Jagitay

use apexcode_core::{JitterEngine, JitterOperation, JitterSchedule};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Main jitter engine implementation
pub struct ApexJitter {
    config: JitterConfig,
    schedule: Arc<Mutex<Option<JitterSchedule>>>,
}

/// Jitter configuration
#[derive(Clone)]
pub struct JitterConfig {
    /// Whether jitter is enabled
    pub enabled: bool,
    /// Minimum delay in seconds
    pub min_delay_secs: u64,
    /// Maximum delay in seconds
    pub max_delay_secs: u64,
}

impl Default for JitterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_delay_secs: 60,
            max_delay_secs: 300,
        }
    }
}

impl ApexJitter {
    /// Create a new jitter engine with default configuration
    pub fn new() -> Self {
        Self::with_config(JitterConfig::default())
    }

    /// Create a new jitter engine with custom configuration
    pub fn with_config(config: JitterConfig) -> Self {
        Self {
            config,
            schedule: Arc::new(Mutex::new(None)),
        }
    }

    /// Calculate a random delay within the configured range
    fn random_delay(&self) -> Duration {
        let mut rng = rand::thread_rng();
        let delay_secs = rng.gen_range(self.config.min_delay_secs..=self.config.max_delay_secs);
        Duration::from_secs(delay_secs)
    }
}

impl JitterEngine for ApexJitter {
    fn apply_jitter(&self, operation: JitterOperation) -> anyhow::Result<JitterSchedule> {
        if !self.config.enabled {
            // If jitter is disabled, execute immediately
            let schedule = JitterSchedule {
                execute_at: chrono::Utc::now(),
                operation,
            };
            *self.schedule.lock().unwrap() = Some(schedule.clone());
            return Ok(schedule);
        }

        let delay = self.random_delay();
        let execute_at = chrono::Utc::now() + chrono::Duration::seconds(delay.as_secs() as i64);

        let schedule = JitterSchedule {
            execute_at,
            operation,
        };

        *self.schedule.lock().unwrap() = Some(schedule.clone());
        Ok(schedule)
    }

    fn current_schedule(&self) -> Option<JitterSchedule> {
        self.schedule.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jitter_creation() {
        let jitter = ApexJitter::new();
        assert!(!jitter.config.enabled);
    }

    #[test]
    fn test_jitter_enabled() {
        let config = JitterConfig {
            enabled: true,
            min_delay_secs: 10,
            max_delay_secs: 20,
        };
        let jitter = ApexJitter::with_config(config);
        assert!(jitter.config.enabled);
    }

    #[test]
    fn test_apply_jitter() {
        let jitter = ApexJitter::new();
        let operation = JitterOperation::Stage {
            files: vec!["test.rs".into()],
        };
        let schedule = jitter.apply_jitter(operation).unwrap();
        assert!(schedule.execute_at <= chrono::Utc::now());
    }
}
