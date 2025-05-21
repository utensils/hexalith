use hexlogogen::web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default port
    let port = 3000;
    
    println!("Starting Hexalith Web Interface on port {}", port);
    web::start_server(port).await?;
    
    Ok(())
}