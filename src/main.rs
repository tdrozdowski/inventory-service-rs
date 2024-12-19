use inventory_service::{start_metrics_server, start_server};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
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
    let otlp_endpoint =
        std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(otlp_endpoint)
                .build()
                .expect("Failed to create exporter"),
            opentelemetry_sdk::runtime::Tokio,
        )
        .build();

    let tracer = provider.tracer("inventory-service");

    // Create a layer with the configured tracer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_versioning=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(otel_layer)
        .init();
    info!("Server initialization started...");
}
