use inventory_service::{start_metrics_server, start_server};
use opentelemetry::global::BoxedTracer;
use opentelemetry::trace::TracerProvider;
use opentelemetry::{
    global,
    trace::{TraceContextExt, TraceError, Tracer},
    KeyValue,
};
use opentelemetry_sdk::{runtime, Resource};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() {
    init();
    info!("Starting server...");
    let (_main_server, _metrics_server) = tokio::join!(start_server(), start_metrics_server());
}

fn init() {
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let tracer = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build()
        .tracer("inventory-service");

    // Create a layer with the configured tracer
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    //let subscriber = Registry::default().with(otel_layer);

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
