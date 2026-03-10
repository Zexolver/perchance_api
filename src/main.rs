use perchance::imagegenerator::ImageGenerator;
use perchance::textgenerator::TextGenerator;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Testing Perchance Text Generator ===");
    let mut text_gen = TextGenerator::new();
    let text_prompt = "How far is the Moon?"; // 
    
    println!("Prompt: {}", text_prompt);
    println!("Generating...");
    
    // This will return the mocked string we set up earlier
    let text_result = text_gen.text(text_prompt).await?;
    println!("Result: {}\n", text_result);


    println!("=== Testing Perchance Image Generator ===");
    let mut image_gen = ImageGenerator::new();
    let image_prompt = "Fantasy landscape"; // 
    
    println!("Prompt: {}", image_prompt);
    println!("Generating...");

    // This will return the mocked ImageResult struct
    let image_result = image_gen.generate(image_prompt, "landscape").await?;
    println!("Generated image mock with ID: {}", image_result.image_id);
    
    // This saves the mock empty byte vector to a file
    let file_name = "mock_output.png";
    image_result.save(Some(file_name)).await?;
    println!("Saved mock image to {}", file_name);

    Ok(())
}
