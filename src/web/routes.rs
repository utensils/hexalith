use crate::generator::Generator;
use crate::svg;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tower_http::{cors::CorsLayer, services::ServeDir};

// Main web interface handler

async fn direct_handler() -> impl IntoResponse {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Hexalith Logo Generator</title>
    <link rel="icon" href="/assets/favicon.svg" type="image/svg+xml">
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
        }
        h1 {
            color: #2b3990;
            margin-bottom: 20px;
        }
        .container {
            display: flex;
            flex-wrap: wrap;
            gap: 20px;
        }
        .controls {
            flex: 1;
            min-width: 300px;
        }
        .preview {
            flex: 1;
            min-width: 300px;
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        .logo-container {
            width: 512px;
            height: 512px;
            border: 1px solid #ddd;
            margin: 20px 0;
            display: flex;
            align-items: center;
            justify-content: center;
            background-color: #f9f9f9;
        }
        form {
            display: flex;
            flex-direction: column;
            gap: 15px;
        }
        .form-group {
            display: flex;
            flex-direction: column;
            gap: 5px;
            margin-bottom: 15px;
        }
        label {
            font-weight: bold;
        }
        select, input[type="number"], input[type="range"] {
            padding: 8px;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        .checkbox-group {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .checkbox-group input {
            width: auto;
        }
        button {
            background-color: #2b3990;
            color: white;
            border: none;
            padding: 10px 15px;
            border-radius: 4px;
            cursor: pointer;
            font-weight: bold;
            margin-top: 10px;
        }
        button:hover {
            background-color: #1e2867;
        }
        .button-group {
            display: flex;
            gap: 10px;
        }
        .button-secondary {
            background-color: #6c757d;
        }
        .button-secondary:hover {
            background-color: #5a6268;
        }
        .range-group {
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .range-group input[type="range"] {
            flex: 1;
        }
        .range-value {
            width: 40px;
            text-align: center;
        }
        footer {
            margin-top: 30px;
            text-align: center;
            color: #666;
            font-size: 0.9rem;
        }
    </style>
</head>
<body>
    <header>
        <h1>Hexalith Logo Generator</h1>
        <p>Create unique hexagonal designs with minimal configuration</p>
    </header>
    
    <div class="container">
        <div class="controls">
            <form id="logo-form">
                <div class="form-group">
                    <label for="theme">Color Theme</label>
                    <select id="theme" name="theme">
                        <option value="mesos" selected>Mesos (Default)</option>
                        <option value="google">Google</option>
                        <option value="blues">Blues</option>
                        <option value="greens">Greens</option>
                        <option value="reds">Reds</option>
                        <option value="purples">Purples</option>
                        <option value="rainbow">Rainbow</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="grid-size">Grid Density (2-8)</label>
                    <div class="range-group">
                        <input type="range" id="grid-size" name="grid_size" min="2" max="8" value="2" step="1">
                        <span id="grid-size-value" class="range-value">2</span>
                    </div>
                </div>
                
                <div class="form-group">
                    <label for="shapes">Number of Shapes (1-10)</label>
                    <div class="range-group">
                        <input type="range" id="shapes" name="shapes" min="1" max="10" value="3" step="1">
                        <span id="shapes-value" class="range-value">3</span>
                    </div>
                </div>
                
                <div class="form-group">
                    <label for="opacity">Opacity (0.0-1.0)</label>
                    <div class="range-group">
                        <input type="range" id="opacity" name="opacity" min="0.1" max="1.0" value="0.8" step="0.1">
                        <span id="opacity-value" class="range-value">0.8</span>
                    </div>
                </div>
                
                <div class="form-group checkbox-group">
                    <input type="checkbox" id="overlap" name="overlap" checked>
                    <label for="overlap">Allow shape overlap</label>
                </div>
                
                <div class="form-group">
                    <label for="seed">Seed (empty = new random seed each time)</label>
                    <input type="text" id="seed" name="seed" value="" placeholder="Enter number to reuse a design">
                </div>
                
                <div class="button-group">
                    <button type="button" id="generate-btn">Generate Logo</button>
                    <button type="button" id="download-btn" class="button-secondary">Download SVG</button>
                </div>
            </form>
        </div>
        
        <div class="preview">
            <div class="logo-container">
                <img id="logo-preview" src="" alt="Generated logo will appear here" style="max-width: 100%; max-height: 100%;">
            </div>
            <div id="logo-info">
                <p>Generate a logo to see information about it here.</p>
            </div>
        </div>
    </div>
    
    <footer>
        <p>Hexalith Logo Generator | Created with 🦀 Rust | <a href="https://github.com/utensils/hexalith">GitHub Repository</a></p>
    </footer>
    
    <script>
        // Function to update range value displays
        function updateRangeValue(rangeInput, valueId, decimals = 0) {
            const value = parseFloat(rangeInput.value);
            document.getElementById(valueId).textContent = value.toFixed(decimals);
        }
        
        // Initialize the page
        document.addEventListener('DOMContentLoaded', function() {
            console.log('Direct HTML page loaded');
            
            // Set up range input event listeners
            document.getElementById('grid-size').addEventListener('input', function() {
                updateRangeValue(this, 'grid-size-value', 0);
            });
            
            document.getElementById('shapes').addEventListener('input', function() {
                updateRangeValue(this, 'shapes-value', 0);
            });
            
            document.getElementById('opacity').addEventListener('input', function() {
                updateRangeValue(this, 'opacity-value', 1);
            });
            
            // Set up button event listeners
            document.getElementById('generate-btn').addEventListener('click', generateLogo);
            document.getElementById('download-btn').addEventListener('click', downloadSvg);
            
            // Generate a random logo on page load
            setTimeout(generateLogo, 300);
        });
        
        // Generate a logo
        async function generateLogo() {
            console.log('Generate logo button clicked');
            const form = document.getElementById('logo-form');
            const formData = new FormData(form);
            
            // Convert FormData to a proper object
            const params = {};
            for (const [key, value] of formData.entries()) {
                if (key === 'grid_size' || key === 'shapes') {
                    params[key] = parseInt(value, 10);
                } else if (key === 'opacity') {
                    params[key] = parseFloat(value);
                } else if (key !== 'overlap' && key !== 'seed') { // Skip overlap and seed for now
                    params[key] = value;
                }
            }
            
            // Handle overlap separately - this approach works
            if (document.getElementById('overlap').checked) {
                params.overlap = true;
            }
            
            // Handle seed parameter - only use a specific seed if provided and valid
            const seedInput = document.getElementById('seed');
            const seedValue = seedInput.value.trim();
            
            if (seedValue !== "" && /^\d+$/.test(seedValue)) {
                // Use the provided seed only if it's a valid number
                params.seed = seedValue;
                // Keep the seed value visible in the input
            } else {
                // Otherwise use a random seed
                delete params.seed;
                // Always clear the seed input field for a new random seed
                seedInput.value = "";
            }
            
            console.log('Form parameters:', params);
            
            try {
                // First get a seed from the server
                const response = await fetch('/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(params)
                });
                
                if (!response.ok) {
                    throw new Error('Failed to generate logo');
                }
                
                const data = await response.json();
                const seed = data.seed;
                console.log('Received seed:', seed);
                
                // Only update the seed field if the user provided a seed
                // Otherwise, keep it clear to ensure a new random seed next time
                if (params.seed) {
                    document.getElementById('seed').value = seed;
                }
                
                // Build query parameters
                const queryParams = new URLSearchParams();
                // Use params object instead of formData to ensure correct types
                for (const [key, value] of Object.entries(params)) {
                    if (value !== undefined && value !== null) {
                        queryParams.append(key, value);
                    }
                }
                
                // Important: Do not include the seed in the URL parameters
                // This prevents browsers from caching the previous SVG
                queryParams.delete('seed');
                
                // Create the URL for the SVG
                const svgUrl = `/svg/${seed}?${queryParams.toString()}`;
                console.log('SVG URL:', svgUrl);
                
                // Update the preview
                const previewImg = document.getElementById('logo-preview');
                previewImg.src = svgUrl;
                
                // Update info
                const logoInfo = document.getElementById('logo-info');
                logoInfo.innerHTML = `
                    <p><strong>Seed:</strong> ${seed}</p>
                    <p><strong>Theme:</strong> ${params.theme}</p>
                    <p><strong>Grid Size:</strong> ${params.grid_size}</p>
                    <p><strong>Shapes:</strong> ${params.shapes}</p>
                    <p><strong>Opacity:</strong> ${params.opacity}</p>
                    <p><strong>Overlap:</strong> ${params.overlap ? 'Yes' : 'No'}</p>
                    <p><strong>Image URL:</strong> <a href="${svgUrl}" target="_blank">View SVG Directly</a></p>
                `;
                
            } catch (error) {
                console.error('Error generating logo:', error);
                alert('Failed to generate logo: ' + error.message);
            }
        }
        
        
        // Download the current SVG
        function downloadSvg() {
            console.log('Download button clicked');
            const previewImg = document.getElementById('logo-preview');
            const svgUrl = previewImg.src;
            
            if (!svgUrl) {
                alert('No logo has been generated yet.');
                return;
            }
            
            // Fetch the SVG content
            fetch(svgUrl)
                .then(response => response.text())
                .then(svgContent => {
                    // Create a blob from the SVG content
                    const blob = new Blob([svgContent], { type: 'image/svg+xml' });
                    const url = URL.createObjectURL(blob);
                    
                    // Create a temporary link and trigger download
                    const a = document.createElement('a');
                    a.href = url;
                    a.download = `hexalith_logo_${document.getElementById('seed').value}.svg`;
                    document.body.appendChild(a);
                    a.click();
                    
                    // Clean up
                    setTimeout(() => {
                        document.body.removeChild(a);
                        URL.revokeObjectURL(url);
                    }, 100);
                })
                .catch(error => {
                    console.error('Error downloading SVG:', error);
                    alert('Failed to download SVG: ' + error.message);
                });
        }
    </script>
</body>
</html>"#;

    (
        axum::http::StatusCode::OK,
        [("Content-Type", "text/html; charset=utf-8")],
        html,
    )
}


pub fn create_router() -> Router {
    // Get the current directory where the binary is running
    let assets_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/web/assets");
    
    // Create the router with our routes
    Router::new()
        .route("/", get(direct_handler)) // Main route with the working interface
        .route("/generate", post(generate_logo_handler))
        .route("/svg/:seed", get(get_svg_handler))
        .route("/favicon.ico", get(favicon_handler))
        .nest_service("/assets", ServeDir::new(assets_path))
        .layer(CorsLayer::permissive())
}


async fn favicon_handler() -> impl IntoResponse {
    // Redirect to the SVG favicon
    (
        axum::http::StatusCode::TEMPORARY_REDIRECT,
        [("Location", "/assets/favicon.svg")],
    )
}

#[derive(Debug, Deserialize)]
struct LogoParams {
    theme: Option<String>,
    shapes: Option<u8>,
    grid_size: Option<u8>,
    opacity: Option<f32>,
    #[serde(default)]
    overlap: Option<bool>, // From JS, it's a boolean
    #[serde(default, deserialize_with = "deserialize_seed")]
    seed: Option<u64>,
}

// Custom deserializer for seed field
fn deserialize_seed<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // This type will catch both string values and null/absent values
    let opt = Option::<String>::deserialize(deserializer)?;
    
    match opt {
        Some(s) if s.is_empty() => Ok(None), // Empty string becomes None
        Some(s) => {
            // Try to parse as u64
            match s.parse::<u64>() {
                Ok(val) => Ok(Some(val)),
                Err(_) => {
                    println!("Failed to parse seed: {}", s);
                    Ok(None) // If it fails to parse, return None
                }
            }
        },
        None => Ok(None) // Null/absent value becomes None
    }
}

#[derive(Debug, Serialize)]
struct LogoResponse {
    seed: u64,
}


// Regular handler with json
async fn generate_logo_handler(
    body: axum::body::Bytes
) -> impl IntoResponse {
    println!("Raw request body: {}", String::from_utf8_lossy(&body));
    
    // Try to parse the request body directly
    let params: LogoParams = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
            return (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                format!("Failed to parse JSON: {}", e)
            ).into_response();
        }
    };
    
    println!("Parsed params: {:?}", params);
    
    // Use the provided seed or generate a random one
    let seed = params.seed.unwrap_or_else(|| {
        use std::time::{SystemTime, UNIX_EPOCH};
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        time ^ 0x12345678 // XOR with a constant for additional randomness
    });
    
    println!("Generated seed: {}", seed);

    (
        axum::http::StatusCode::OK,
        axum::Json(LogoResponse { seed })
    ).into_response()
}

async fn get_svg_handler(Path(seed): Path<u64>, Query(params): Query<LogoParams>) -> impl IntoResponse {
    // Set up the generator with the parameters from the query string
    let grid_size = params.grid_size.unwrap_or(2);
    let shapes = params.shapes.unwrap_or(3);
    let opacity = params.opacity.unwrap_or(0.8);
    let theme = params.theme.unwrap_or_else(|| "mesos".to_string());
    // For the direct HTML version, overlap is now a boolean
    let overlap = params.overlap.unwrap_or(true);
    
    // Debug output to server console
    println!("Generating logo with: seed={}, grid_size={}, shapes={}, opacity={}, theme={}, overlap={}", 
        seed, grid_size, shapes, opacity, theme, overlap);

    // Create the generator
    let mut generator = Generator::new(grid_size, shapes, opacity, Some(seed));
    generator
        .set_color_scheme(&theme)
        .set_allow_overlap(overlap);

    // Generate the logo
    if let Err(e) = generator.generate() {
        println!("Error generating logo: {}", e);
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error generating logo: {}", e),
        ).into_response();
    }
    
    println!("Logo generation successful, generated {} shapes", generator.shapes().len());

    // Generate SVG
    match svg::generate_svg(&generator, 512, 512) {
        Ok(svg_data) => {
            println!("SVG generation successful, size: {} bytes", svg_data.len());
            (
                axum::http::StatusCode::OK,
                [
                    ("Content-Type", "image/svg+xml"),
                    ("Cache-Control", "public, max-age=86400"), // Cache for a day
                ],
                svg_data,
            ).into_response()
        }
        Err(e) => {
            println!("Error generating SVG: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error generating SVG: {}", e),
            ).into_response()
        }
    }
}