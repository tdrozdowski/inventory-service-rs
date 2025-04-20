use inventory_service::{start_metrics_server, start_server};
use opentelemetry::logs::LoggerProvider;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
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
    let otlp_endpoint =
        std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());
    let log_endpoint = otlp_endpoint.clone();
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_endpoint)
        .build()
        .expect("Failed to create exporter");

    let provider = opentelemetry_sdk::trace::TracerProviderBuilder::default()
        .with_batch_exporter(exporter)
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", "inventory_service"))
                .build(),
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

    // setup profiling
    // Configure profiling backend
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
