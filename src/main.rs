use inventory_service::{start_metrics_server, start_server, PYROSCOPE_AGENT};
use opentelemetry::logs::LoggerProvider;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use pyroscope::PyroscopeAgent;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use std::sync::{Arc, OnceLock};
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

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

    // Service name configuration
    let service_name =
        std::env::var("SERVICE_NAME").unwrap_or_else(|_| "inventory_service_local".to_string());

    // Initialize the global PyroscopeAgent
    let pyroscope_endpoint = std::env::var("PYROSCOPE_ENDPOINT")
        .unwrap_or_else(|_| "http://pyroscope-1746333570.pyroscope.local:4040".to_string());

    // Create a backend implementation for the PyroscopeAgent
    let backend_impl = pprof_backend(PprofConfig::new().sample_rate(100));

    // Build and initialize the PyroscopeAgent
    let agent = pyroscope::PyroscopeAgent::builder(pyroscope_endpoint, service_name.clone())
        .backend(backend_impl)
        .tags(vec![("environment", "production")])
        .build()
        .expect("Failed to create Pyroscope agent");

    // Start the agent and store it in the global PYROSCOPE_AGENT OnceCell
    let running_agent = agent.start().expect("Failed to start Pyroscope agent");
    inventory_service::PYROSCOPE_AGENT
        .set(running_agent)
        .expect("Failed to set global Pyroscope agent");

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
                .with_attribute(KeyValue::new("service.name", service_name.clone()))
                .build(),
        )
        .build();

    let tracer = tracer_provider.tracer("inventory-service");

    // Configure the OTLP log exporter
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http() // Use HTTP transport instead of gRPC
        .with_endpoint(otlp_endpoint.replace("4317", "4318")) // Use HTTP port 4318 instead of gRPC port 4317
        .build()
        .expect("Failed to create OTLP log exporter");

    let log_provider = opentelemetry_sdk::logs::SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", service_name.clone()))
                .build(),
        )
        .build();
    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("opentelemetry=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_logger_layer =
        layer::OpenTelemetryTracingBridge::new(&log_provider).with_filter(filter_otel);

    // Fixed OTLP log exporter error "unknown service opentelemetry.proto.collector.logs.v1.LogsService"
    // by switching from gRPC (port 4317) to HTTP (port 4318) protocol for both trace and log exporters

    // Create a tracing layer for exporting traces
    let otel_tracer_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Create a filter for the main subscriber that filters out h2 logs and opentelemetry_sdk debug logs
    let main_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "example_versioning=debug".into())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("opentelemetry_sdk=info".parse().unwrap()); // Suppress DEBUG logs from opentelemetry_sdk

    // Compile the tracing subscriber with OTLP trace layer only
    tracing_subscriber::registry()
        .with(main_filter)
        .with(tracing_subscriber::fmt::layer()) // For local debugging
        .with(otel_tracer_layer) // Add OTLP trace layer
        .with(otel_logger_layer) // Add OTLP log layer
        .init();

    info!("Server initialization started...");
}
