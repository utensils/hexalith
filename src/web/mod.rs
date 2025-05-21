pub mod routes;
pub mod templates;
pub mod templates_new;

use crate::Result;

pub async fn start_server(port: u16) -> Result<()> {
    let app = routes::create_router();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Web server running at http://localhost:{}", port);

    axum::serve(listener, app).await?;
    Ok(())
}