use inventory_service::{start_metrics_server, start_server};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use std::sync::{Arc, OnceLock};
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

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

    // Pyroscope agent initialization has been removed to avoid conflicts with per-request profiling
    // The per-request profiling is now handled in lib.rs via the profile_request middleware

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

    // Removed OTLP log exporter to avoid the repeating error:
    // "unknown service opentelemetry.proto.collector.logs.v1.LogsService"
    // We'll rely on the standard tracing subscriber for logs instead

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
        .init();

    info!("Server initialization started...");
}
