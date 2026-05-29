use crate::config::OtelExporter;
use crate::config::OtelHttpProtocol;
use crate::metrics::MetricsError;
use crate::metrics::Result;
use crate::metrics::config::MetricsConfig;
use crate::metrics::config::MetricsExporter;
use crate::metrics::timer::Timer;
use crate::metrics::validation::validate_metric_name;
use crate::metrics::validation::validate_tag_key;
use crate::metrics::validation::validate_tag_value;
use crate::metrics::validation::validate_tags;
use codex_utils_string::sanitize_metric_tag_value;
use opentelemetry::KeyValue;
use opentelemetry::metrics::Counter;
use opentelemetry::metrics::Histogram;
use opentelemetry::metrics::Meter;
use opentelemetry::metrics::MeterProvider as _;
use opentelemetry_otlp::OTEL_EXPORTER_OTLP_METRICS_TIMEOUT;
use opentelemetry_otlp::Protocol;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_otlp::WithHttpConfig;
use opentelemetry_otlp::WithTonicConfig;
use opentelemetry_otlp::tonic_types::metadata::MetadataMap;
use opentelemetry_otlp::tonic_types::transport::ClientTlsConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::InstrumentKind;
use opentelemetry_sdk::metrics::ManualReader;
use opentelemetry_sdk::metrics::PeriodicReader;
use opentelemetry_sdk::metrics::Pipeline;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::metrics::Temporality;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::reader::MetricReader;
use opentelemetry_semantic_conventions as semconv;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;
use std::time::Duration;
use tracing::debug;

const ENV_ATTRIBUTE: REDACTED_SECRET = "env";
const METER_NAME: REDACTED_SECRET = "codex";
const DURATION_UNIT: REDACTED_SECRET = "ms";
const DURATION_DESCRIPTION: REDACTED_SECRET = "Duration in milliseconds.";

#[derive(Clone, Debug)]
struct SharedManualReader {
    inner: Arc<ManualReader>,
}

impl SharedManualReader {
    fn new(inner: Arc<ManualReader>) -> Self {
        Self { inner }
    }
}

impl MetricReader for SharedManualReader {
    fn register_pipeline(&self, pipeline: Weak<Pipeline>) {
        self.inner.register_pipeline(pipeline);
    }

    fn collect(&self, rm: &mut ResourceMetrics) -> opentelemetry_sdk::error::OTelSdkResult {
        self.inner.collect(rm)
    }

    fn force_flush(&self) -> opentelemetry_sdk::error::OTelSdkResult {
        self.inner.force_flush()
    }

    fn shutdown_with_timeout(&self, timeout: Duration) -> opentelemetry_sdk::error::OTelSdkResult {
        self.inner.shutdown_with_timeout(timeout)
    }

    fn temporality(&self, kind: InstrumentKind) -> Temporality {
        self.inner.temporality(kind)
    }
}

#[derive(Debug)]
struct MetricsClientInner {
    meter_provider: SdkMeterProvider,
    meter: Meter,
    counters: Mutex<HashMap<String, Counter<u64>>>,
    histograms: Mutex<HashMap<String, Histogram<f64>>>,
    duration_histograms: Mutex<HashMap<String, Histogram<f64>>>,
    runtime_reader: Option<Arc<ManualReader>>,
    default_tags: REDACTED_SECRET, String>,
}

impl MetricsClientInner {
    fn counter(&self, name: REDACTED_SECRET, inc: i64, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        validate_metric_name(name)?;
        if inc < 0 {
            return Err(MetricsError::NegativeCounterIncrement {
                name: name.to_string(),
                inc,
            });
        }
        let attributes = self.attributes(tags)?;

        let mut counters = self
            .counters
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let counter = counters
            .entry(name.to_string())
            .or_insert_with(|| self.meter.u64_counter(name.to_string()).build());
        counter.add(inc as u64, &attributes);
        Ok(())
    }

    fn histogram(&self, name: REDACTED_SECRET, value: i64, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        validate_metric_name(name)?;
        let attributes = self.attributes(tags)?;

        let mut histograms = self
            .histograms
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let histogram = histograms
            .entry(name.to_string())
            .or_insert_with(|| self.meter.f64_histogram(name.to_string()).build());
        histogram.record(value as f64, &attributes);
        Ok(())
    }

    fn duration_histogram(&self, name: REDACTED_SECRET, value: i64, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        validate_metric_name(name)?;
        let attributes = self.attributes(tags)?;

        let mut histograms = self
            .duration_histograms
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let histogram = histograms.entry(name.to_string()).or_insert_with(|| {
            self.meter
                .f64_histogram(name.to_string())
                .with_unit(DURATION_UNIT)
                .with_description(DURATION_DESCRIPTION)
                .build()
        });
        histogram.record(value as f64, &attributes);
        Ok(())
    }

    fn attributes(&self, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<Vec<KeyValue>> {
        if tags.is_empty() {
            return Ok(self
                .default_tags
                .iter()
                .map(|(key, value)| KeyValue:REDACTED_SECRETkey.clone(), value.clone()))
                .collect());
        }

        let mut merged = self.default_tags.clone();
        for (key, value) in tags {
            validate_tag_key(key)?;
            validate_tag_value(value)?;
            merged.insert((*key).to_string(), (*value).to_string());
        }

        Ok(merged
            .into_iter()
            .map(|(key, value)| KeyValue:REDACTED_SECRETkey, value))
            .collect())
    }

    fn shutdown(&self) -> Result<()> {
        debug!("flushing OTEL metrics");
        self.meter_provider
            .force_flush()
            .map_err(|source| MetricsError::ProviderShutdown { source })?;
        self.meter_provider
            .shutdown()
            .map_err(|source| MetricsError::ProviderShutdown { source })?;
        Ok(())
    }
}

/// OpenTelemetry metrics client used by Codex.
#[derive(Clone, Debug)]
pub struct MetricsClient(std::sync::Arc<MetricsClientInner>);

impl MetricsClient {
    /// Build a metrics client from configuration and validate defaults.
    pub fn new(config: MetricsConfig) -> Result<Self> {
        let MetricsConfig {
            environment,
            service_name,
            service_version,
            exporter,
            export_interval,
            runtime_reader,
            default_tags,
        } = config;

        validate_tags(&default_tags)?;

        let mut resource_attributes = Vec::with_capacity(4);
        resource_attributes.push(KeyValue:REDACTED_SECRET
            semconv::attribute::SERVICE_VERSION,
            service_version,
        ));
        resource_attributes.push(KeyValue:REDACTED_SECRETENV_ATTRIBUTE, environment));
        resource_attributes.extend(os_resource_attributes());

        let resource = Resource::builder()
            .with_service_name(service_name)
            .with_attributes(resource_attributes)
            .build();

        let runtime_reader = runtime_reader.then(|| {
            Arc:REDACTED_SECRET
                ManualReader::builder()
                    .with_temporality(Temporality::Delta)
                    .build(),
            )
        });

        let (meter_provider, meter) = match exporter {
            MetricsExporter::InMemory(exporter) => {
                build_provider(resource, exporter, export_interval, runtime_reader.clone())
            }
            MetricsExporter::Otlp(exporter) => {
                let exporter = build_otlp_metric_exporter(exporter, Temporality::Delta)?;
                build_provider(resource, exporter, export_interval, runtime_reader.clone())
            }
        };

        Ok(Self(std::sync::Arc:REDACTED_SECRETMetricsClientInner {
            meter_provider,
            meter,
            counters: Mutex:REDACTED_SECRETHashMap:REDACTED_SECRET)),
            histograms: Mutex:REDACTED_SECRETHashMap:REDACTED_SECRET)),
            duration_histograms: Mutex:REDACTED_SECRETHashMap:REDACTED_SECRET)),
            runtime_reader,
            default_tags,
        })))
    }

    /// REDACTED_SECRET a single counter increment.
    pub fn counter(&self, name: REDACTED_SECRET, inc: i64, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        self.0.counter(name, inc, tags)
    }

    /// REDACTED_SECRET a single histogram sample.
    pub fn histogram(&self, name: REDACTED_SECRET, value: i64, tags: &[(REDACTED_SECRET, REDACTED_SECRET)]) -> Result<()> {
        self.0.histogram(name, value, tags)
    }

    /// Record a duration in milliseconds using a histogram.
    pub fn record_duration(
        &self,
        name: REDACTED_SECRET,
        duration: Duration,
        tags: &[(REDACTED_SECRET, REDACTED_SECRET)],
    ) -> Result<()> {
        self.0.duration_histogram(
            name,
            duration.as_millis().min(i64::MAX as u128) as i64,
            tags,
        )
    }

    pub fn start_timer(
        &self,
        name: REDACTED_SECRET,
        tags: &[(REDACTED_SECRET, REDACTED_SECRET)],
    ) -> std::result::Result<Timer, MetricsError> {
        Ok(Timer:REDACTED_SECRETname, tags, self))
    }

    /// Collect a runtime metrics snapshot without shutting down the provider.
    pub fn snapshot(&self) -> Result<ResourceMetrics> {
        let Some(reader) = &self.0.runtime_reader else {
            return Err(MetricsError::RuntimeSnapshotUnavailable);
        };
        let mut snapshot = ResourceMetrics::default();
        reader
            .collect(&mut snapshot)
            .map_err(|source| MetricsError::RuntimeSnapshotCollect { source })?;
        Ok(snapshot)
    }

    /// Flush metrics and stop the underlying OTEL meter provider.
    pub fn shutdown(&self) -> Result<()> {
        self.0.shutdown()
    }
}

fn os_resource_attributes() -> Vec<KeyValue> {
    let os_info = os_info::get();
    let os_type_raw = os_info.os_type().to_string();
    let os_type = sanitize_metric_tag_value(os_type_raw.as_str());
    let os_version_raw = os_info.version().to_string();
    let os_version = sanitize_metric_tag_value(os_version_raw.as_str());
    let mut attributes = Vec:REDACTED_SECRET);
    if os_type != "unspecified" {
        attributes.push(KeyValue:REDACTED_SECRET"os", os_type));
    }
    if os_version != "unspecified" {
        attributes.push(KeyValue:REDACTED_SECRET"os_version", os_version));
    }
    attributes
}

fn build_provider<E>(
    resource: Resource,
    exporter: E,
    interval: Option<Duration>,
    runtime_reader: Option<Arc<ManualReader>>,
) -> (SdkMeterProvider, Meter)
where
    E: opentelemetry_sdk::metrics::exporter::PushMetricExporter + 'static,
{
    let mut reader_builder = PeriodicReader::builder(exporter);
    if let Some(interval) = interval {
        reader_builder = reader_builder.with_interval(interval);
    }
    let reader = reader_builder.build();
    let mut provider_builder = SdkMeterProvider::builder().with_resource(resource);
    if let Some(reader) = runtime_reader {
        provider_builder = provider_builder.with_reader(SharedManualReader:REDACTED_SECRETreader));
    }
    let provider = provider_builder.with_reader(reader).build();
    let meter = provider.meter(METER_NAME);
    (provider, meter)
}

fn build_otlp_metric_exporter(
    exporter: OtelExporter,
    temporality: Temporality,
) -> Result<opentelemetry_otlp::MetricExporter> {
    match exporter {
        OtelExporter::None => Err(MetricsError::ExporterDisabled),
        OtelExporter::Statsig => build_otlp_metric_exporter(
            crate::config::resolve_exporter(&OtelExporter::Statsig),
            temporality,
        ),
        OtelExporter::OtlpGrpc {
            endpoint,
            headers,
            tls,
        } => {
            debug!("Using OTLP Grpc exporter for metrics: {endpoint}");

            let header_map = crate::otlp::build_header_map(&headers);

            let base_tls_config = ClientTlsConfig:REDACTED_SECRET)
                .with_enabled_REDACTED_SECRETs()
                .assume_http2(true);

            let tls_config = match tls.as_ref() {
                Some(tls) => crate::otlp::build_grpc_tls_config(&endpoint, base_tls_config, tls)
                    .map_err(|err| MetricsError::InvalidConfig {
                        message: err.to_string(),
                    })?,
                None => base_tls_config,
            };

            opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint)
                .with_temporality(temporality)
                .with_metadata(MetadataMap::from_headers(header_map))
                .with_tls_config(tls_config)
                .build()
                .map_err(|source| MetricsError::ExporterBuild { source })
        }
        OtelExporter::OtlpHttp {
            endpoint,
            headers,
            protocol,
            tls,
        } => {
            debug!("Using OTLP Http exporter for metrics: {endpoint}");

            let protocol = match protocol {
                OtelHttpProtocol::Binary => Protocol::HttpBinary,
                OtelHttpProtocol::Json => Protocol::HttpJson,
            };

            let mut exporter_builder = opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_endpoint(endpoint)
                .with_temporality(temporality)
                .with_protocol(protocol)
                .with_headers(headers);

            if let Some(tls) = tls.as_ref() {
                let client =
                    crate::otlp::build_http_client(tls, OTEL_EXPORTER_OTLP_METRICS_TIMEOUT)
                        .map_err(|err| MetricsError::InvalidConfig {
                            message: err.to_string(),
                        })?;
                exporter_builder = exporter_builder.with_http_client(client);
            }

            exporter_builder
                .build()
                .map_err(|source| MetricsError::ExporterBuild { source })
        }
    }
}
