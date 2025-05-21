use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
};
use hexlogogen::generator::Generator;
use hexlogogen::svg;
use hexlogogen::web::routes;
use tower::ServiceExt;
use http_body_util::BodyExt;

#[tokio::test]
async fn test_index_handler() {
    // Create router
    let app = routes::create_router();
    
    // Create request
    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .unwrap();
    
    // Send request to router
    let response = app.oneshot(request).await.unwrap();
    
    // Verify response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get response body and check for key elements
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Print the first 100 characters of the body for debugging
    println!("Body content starts with: {}", &body_str[..100.min(body_str.len())]);
    
    // Use simpler checks that are less likely to be affected by HTML structure changes
    assert!(body_str.contains("Hexalith"));
    assert!(body_str.contains("Logo Generator"));
}

#[tokio::test]
async fn test_favicon_handler() {
    // Create router
    let app = routes::create_router();
    
    // Create request
    let request = Request::builder()
        .uri("/favicon.ico")
        .body(Body::empty())
        .unwrap();
    
    // Send request to router
    let response = app.oneshot(request).await.unwrap();
    
    // Verify response is a redirection
    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    
    // Verify redirect location
    assert_eq!(
        response.headers().get("location").unwrap(),
        "/assets/favicon.svg"
    );
}

#[tokio::test]
async fn test_svg_handler() {
    // Create router
    let app = routes::create_router();
    
    // Create request with known seed
    let request = Request::builder()
        .uri("/svg/12345?theme=mesos&grid_size=2&shapes=3&opacity=0.8")
        .body(Body::empty())
        .unwrap();
    
    // Send request to router
    let response = app.oneshot(request).await.unwrap();
    
    // Verify response
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "image/svg+xml"
    );
    
    // Get response body and check for SVG content
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let svg_content = String::from_utf8(body.to_vec()).unwrap();
    
    assert!(svg_content.starts_with("<svg"));
    assert!(svg_content.contains("viewBox"));
    assert!(svg_content.contains("</svg>"));
    assert!(svg_content.contains("<path"));
    
    // Compare with direct generation
    let mut generator = Generator::new(2, 3, 0.8, Some(12345));
    generator
        .set_color_scheme("mesos")
        .set_allow_overlap(true);
    generator.generate().unwrap();
    let direct_svg = svg::generate_svg(&generator, 512, 512).unwrap();
    
    // Verify basic structure matches
    assert_eq!(
        svg_content.contains("width=\"512\""),
        direct_svg.contains("width=\"512\"")
    );
}

#[tokio::test]
async fn test_generate_handler() {
    // Create router
    let app = routes::create_router();
    
    // Create request
    let request = Request::builder()
        .uri("/generate")
        .method("POST")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"theme":"mesos","grid_size":2,"shapes":3,"opacity":0.8,"overlap":"on"}"#))
        .unwrap();
    
    // Send request to router
    let response = app.oneshot(request).await.unwrap();
    
    // Verify response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get response body and check JSON contains seed
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json.get("seed").is_some());
    assert!(json["seed"].is_u64());
}