use hexlogogen::Generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new generator with default settings
    let generator = Generator::new(6, 3, 0.8, None);
    
    // Generate a logo
    generator.generate()?;
    
    println!("Logo generated successfully!");
    Ok(())
}