mod app;
mod core;
mod bootstrap;
mod routes;
use app::providers::app_service_provider::AppServiceProvider;

#[tokio::main]
async fn main() {
    let app_state = AppServiceProvider::boot();
    let app = bootstrap::create_app(app_state.clone());
    let host = app_state.app.url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let addr = format!("{}:{}", host, app_state.app.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("✅ {} started on http://{}", app_state.app.name, addr);
    axum::serve(listener, app).await.unwrap();
}
