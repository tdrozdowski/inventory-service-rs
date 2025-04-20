use inventory_service::{start_metrics_server, start_server};
use opentelemetry::logs::LoggerProvider;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    init().await;
    info!("Starting server...");
    let (_main_server, _metrics_server) = tokio::join!(start_server(), start_metrics_server());
}

async fn init() {
    // OTLP endpoint configuration
    let otlp_endpoint =
        std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());

    // Configure the OTLP trace exporter and tracer provider
    let trace_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic() // Use gRPC transport
        .with_endpoint(otlp_endpoint.clone())
        .build()
        .expect("Failed to create OTLP trace exporter");

    let tracer_provider = opentelemetry_sdk::trace::TracerProviderBuilder::default()
        .with_batch_exporter(trace_exporter)
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", "inventory_service"))
                .build(),
        )
        .build();

    let tracer = tracer_provider.tracer("inventory-service");

    // Configure the OTLP log exporter
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic() // Use gRPC transport
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to create OTLP log exporter");

    let log_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", "inventory_service"))
                .build(),
        )
        .build();

    let otel_logger_layer = layer::OpenTelemetryTracingBridge::new(&log_provider); //.with_filter(filter_otel);

    // Create a tracing layer for exporting traces
    let otel_tracer_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Compile the tracing subscriber with OTLP layers
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_versioning=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer()) // For local debugging
        .with(otel_tracer_layer) // Add OTLP trace layer
        .with(otel_logger_layer) // Add OTLP log layer
        .init();

    // Profiling setup
    let pprof_config = PprofConfig::new().sample_rate(100);
    let backend_impl = pprof_backend(pprof_config);

    // Configure Pyroscope Agent
    let pyro_endpoint = std::env::var("PYROSCOPE_ENDPOINT")
        .unwrap_or_else(|_| "http://pyroscope-ingester.pyroscope:4040".to_string());
    let agent = PyroscopeAgent::builder(pyro_endpoint.as_str(), "inventory_service")
        .backend(backend_impl)
        .build()
        .expect("Failed to create Pyroscope agent");

    info!("Server initialization started...");
}
