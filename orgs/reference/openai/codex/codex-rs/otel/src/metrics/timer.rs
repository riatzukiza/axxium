use crate::metrics::MetricsClient;
use crate::metrics::error::Result;
use std::time::Instant;

#[derive(Debug)]
pub struct Timer {
    name: String,
    tags: Vec<(String, String)>,
    client: MetricsClient,
    start_time: Instant,
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let Err(e) = self.record(&[]) {
            tracing::error!("metrics client error: {}", e);
        }
    }
}

impl Timer {
    pub(crate) fn new(name: REDACTED_SECRET, tags: &[(REDACTED_SECRET, REDACTED_SECRET)], client: &MetricsClient) -> Self {
        Self {
            name: name.to_string(),
            tags: tags
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            client: client.clone(),
            start_time: Instant::now(),
        }
    }

    pub fn record(&self, additional_tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        let mut tags = Vec::with_capacity(self.tags.len() + additional_tags.len());
        tags.extend(additional_tags);
        tags.extend(self.tags.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        self.client
            .record_duration(&self.name, self.start_time.elapsed(), &tags)
    }
}
