use maud::{html, Markup};

pub fn index_page() -> Markup {
    html! {
        (maud::DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="icon" href="/assets/favicon.svg" type="image/svg+xml";
                title { "Hexalith Logo Generator" }
                style { r#"
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
                    .history {
                        margin-top: 30px;
                    }
                    .history h2 {
                        border-bottom: 1px solid #ddd;
                        padding-bottom: 5px;
                    }
                    .history-grid {
                        display: grid;
                        grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
                        gap: 10px;
                        margin-top: 15px;
                    }
                    .history-item {
                        width: 100px;
                        height: 100px;
                        border: 1px solid #ddd;
                        cursor: pointer;
                        overflow: hidden;
                    }
                    .history-item img {
                        width: 100%;
                        height: 100%;
                        object-fit: contain;
                    }
                    .history-item:hover {
                        border-color: #2b3990;
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
                    .button-secondary {
                        background-color: #6c757d;
                    }
                    .button-secondary:hover {
                        background-color: #5a6268;
                    }
                    "#
                }
            }
            body {
                header {
                    h1 { "Hexalith Logo Generator" }
                    p { "Create unique hexagonal designs with minimal configuration" }
                }

                div class="container" {
                    div class="controls" {
                        form id="logo-form" {
                            div class="form-group" {
                                label for="theme" { "Color Theme" }
                                select id="theme" name="theme" {
                                    option value="mesos" selected { "Mesos (Default)" }
                                    option value="google" { "Google" }
                                    option value="blues" { "Blues" }
                                    option value="greens" { "Greens" }
                                    option value="reds" { "Reds" }
                                    option value="purples" { "Purples" }
                                    option value="rainbow" { "Rainbow" }
                                }
                            }

                            div class="form-group" {
                                label for="grid-size" { "Grid Density (2-8)" }
                                div class="range-group" {
                                    input type="range" id="grid-size" name="grid_size" min="2" max="8" value="2" step="1" {}
                                    span id="grid-size-value" class="range-value" { "2" }
                                }
                            }

                            div class="form-group" {
                                label for="shapes" { "Number of Shapes (1-10)" }
                                div class="range-group" {
                                    input type="range" id="shapes" name="shapes" min="1" max="10" value="3" step="1" {}
                                    span id="shapes-value" class="range-value" { "3" }
                                }
                            }

                            div class="form-group" {
                                label for="opacity" { "Opacity (0.0-1.0)" }
                                div class="range-group" {
                                    input type="range" id="opacity" name="opacity" min="0.1" max="1.0" value="0.8" step="0.1" {}
                                    span id="opacity-value" class="range-value" { "0.8" }
                                }
                            }

                            div class="form-group checkbox-group" {
                                input type="checkbox" id="overlap" name="overlap" checked {}
                                label for="overlap" { "Allow shape overlap" }
                            }

                            input type="hidden" id="seed" name="seed" value="" {}

                            div class="button-group" {
                                button type="button" id="generate-btn" { "Generate Random" }
                                button type="button" id="download-btn" class="button-secondary" { "Download SVG" }
                            }
                        }
                    }

                    div class="preview" {
                        div class="logo-container" {
                            img id="logo-preview" src="" alt="Generated logo will appear here" {}
                        }
                        div id="logo-info" {
                            p { "Generate a logo to see information about it here." }
                        }
                    }
                }

                div class="history" {
                    h2 { "Recent Logos" }
                    div id="history-grid" class="history-grid" {}
                }

                footer {
                    p { "Hexalith Logo Generator | Created with 🦀 Rust | " a href="https://github.com/utensils/hexalith" { "GitHub Repository" } }
                }

                script { r#"
                    // Store recent logos in local storage
                    let recentLogos = JSON.parse(localStorage.getItem('hexalith_recent_logos') || '[]');
                    const MAX_HISTORY = 20;
                    
                    // Function to update range value displays
                    function updateRangeValue(rangeInput, valueId, decimals = 0) {
                        const value = parseFloat(rangeInput.value);
                        document.getElementById(valueId).textContent = value.toFixed(decimals);
                    }
                    
                    // Initialize the page
                    document.addEventListener('DOMContentLoaded', function() {
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
                        
                        // Show logos from history
                        updateHistoryGrid();
                        
                        // Generate a random logo on page load
                        setTimeout(generateLogo, 100);
                    });
                    
                    // Generate a logo
                    async function generateLogo() {
                        const form = document.getElementById('logo-form');
                        const formData = new FormData(form);
                        
                        // Convert FormData to a proper object with correct types
                        const params = {};
                        for (const [key, value] of formData.entries()) {
                            // Convert numeric values to numbers
                            if (key === 'grid_size' || key === 'shapes') {
                                params[key] = parseInt(value, 10);
                            } else if (key === 'opacity') {
                                params[key] = parseFloat(value);
                            } else if (key !== 'overlap') { // Skip overlap for now
                                params[key] = value;
                            }
                        }
                        
                        // Handle overlap separately - this is the key fix that works
                        if (document.getElementById('overlap').checked) {
                            params.overlap = true;
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
                            
                            // Update the seed field
                            document.getElementById('seed').value = seed;
                            
                            // Build query parameters
                            const queryParams = new URLSearchParams();
                            for (const [key, value] of formData.entries()) {
                                // Skip undefined or null values
                                if (value !== undefined && value !== null) {
                                    queryParams.append(key, value);
                                }
                            }
                            
                            // Create the URL for the SVG
                            const svgUrl = `/svg/${seed}?${queryParams.toString()}`;
                            
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
                            
                            // Add to history
                            addToHistory({
                                seed,
                                theme: params.theme,
                                grid_size: params.grid_size,
                                shapes: params.shapes,
                                opacity: params.opacity,
                                overlap: params.overlap === true,
                                url: svgUrl
                            });
                            
                        } catch (error) {
                            console.error('Error generating logo:', error);
                            alert('Failed to generate logo: ' + error.message);
                        }
                    }
                    
                    // Add a logo to history
                    function addToHistory(logo) {
                        // Check if this exact logo is already in history
                        const existingIndex = recentLogos.findIndex(l => l.seed === logo.seed);
                        if (existingIndex >= 0) {
                            // Remove it so we can add it to the front
                            recentLogos.splice(existingIndex, 1);
                        }
                        
                        // Add to front of array
                        recentLogos.unshift(logo);
                        
                        // Limit the number of saved logos
                        if (recentLogos.length > MAX_HISTORY) {
                            recentLogos = recentLogos.slice(0, MAX_HISTORY);
                        }
                        
                        // Save to local storage
                        localStorage.setItem('hexalith_recent_logos', JSON.stringify(recentLogos));
                        
                        // Update the display
                        updateHistoryGrid();
                    }
                    
                    // Update the history grid display
                    function updateHistoryGrid() {
                        const historyGrid = document.getElementById('history-grid');
                        historyGrid.innerHTML = '';
                        
                        recentLogos.forEach(logo => {
                            const item = document.createElement('div');
                            item.className = 'history-item';
                            item.title = `Theme: ${logo.theme}, Grid: ${logo.grid_size}, Shapes: ${logo.shapes}`;
                            
                            const img = document.createElement('img');
                            img.src = logo.url;
                            img.alt = 'Saved logo';
                            
                            item.appendChild(img);
                            
                            // Add click handler to load this logo
                            item.addEventListener('click', () => {
                                loadLogoFromHistory(logo);
                            });
                            
                            historyGrid.appendChild(item);
                        });
                    }
                    
                    // Load a logo from history
                    function loadLogoFromHistory(logo) {
                        // Update form values
                        document.getElementById('theme').value = logo.theme;
                        document.getElementById('grid-size').value = logo.grid_size;
                        document.getElementById('grid-size-value').textContent = logo.grid_size;
                        document.getElementById('shapes').value = logo.shapes;
                        document.getElementById('shapes-value').textContent = logo.shapes;
                        document.getElementById('opacity').value = logo.opacity;
                        document.getElementById('opacity-value').textContent = logo.opacity;
                        document.getElementById('overlap').checked = logo.overlap;
                        document.getElementById('seed').value = logo.seed;
                        
                        // Update preview
                        document.getElementById('logo-preview').src = logo.url;
                        
                        // Update info
                        const logoInfo = document.getElementById('logo-info');
                        logoInfo.innerHTML = `
                            <p><strong>Seed:</strong> ${logo.seed}</p>
                            <p><strong>Theme:</strong> ${logo.theme}</p>
                            <p><strong>Grid Size:</strong> ${logo.grid_size}</p>
                            <p><strong>Shapes:</strong> ${logo.shapes}</p>
                            <p><strong>Opacity:</strong> ${logo.opacity}</p>
                            <p><strong>Overlap:</strong> ${logo.overlap ? 'Yes' : 'No'}</p>
                        `;
                    }
                    
                    // Download the current SVG
                    function downloadSvg() {
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
                    "#
                }
            }
        }
    }
}
