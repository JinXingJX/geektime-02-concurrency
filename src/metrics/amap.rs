use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc},
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key not found,key:{}", key))?;
        counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data),
        }
    }
}
