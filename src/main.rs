// This is the entry point of our program
// It imports the tokenizer module and shows how to use it

// Declare that we have a module called 'tokenizer' in tokenizer.rs
mod tokenizer;

// Import the Tokenizer struct from our tokenizer module
use tokenizer::Tokenizer;

fn main() {
    // Print a welcoming header
    println!("╔════════════════════════════════════════╗");
    println!("║     Rust Tokenizer - Main Program      ║");
    println!("╚════════════════════════════════════════╝\n");
    
    // EXAMPLE 1: Simple sentence tokenization
    println!("📌 EXAMPLE 1: Simple Tokenization");
    println!("─────────────────────────────────────\n");
    
    // Create a sample sentence
    let sentence1 = "Hello, world! How are you?";
    println!("Original sentence: \"{}\"", sentence1);
    
    // Create a new Tokenizer instance with our sentence
    // The Tokenizer::new() function takes ownership of the string
    let tokenizer1 = Tokenizer::new(sentence1.to_string());
    
    // Get the tokens (this breaks the sentence into pieces)
    let tokens1 = tokenizer1.tokenize();
    println!("Tokens: {:?}", tokens1);
    
    // Reconstruct the original sentence from tokens
    // detokenize() uses smart rules to handle spacing with punctuation
    let reconstructed1 = tokenizer1.detokenize(&tokens1);
    println!("Reconstructed: \"{}\"", reconstructed1);
    println!("Match original? {}\n", reconstructed1 == sentence1);
    
    // EXAMPLE 2: Complex sentence with various punctuation
    println!("📌 EXAMPLE 2: Complex Punctuation");
    println!("─────────────────────────────────────\n");
    
    let sentence2 = "Mr. Smith said, \"Don't worry!\"";
    println!("Original sentence: \"{}\"", sentence2);
    
    let tokenizer2 = Tokenizer::new(sentence2.to_string());
    let tokens2 = tokenizer2.tokenize();
    println!("Tokens: {:?}", tokens2);
    
    let reconstructed2 = tokenizer2.detokenize(&tokens2);
    println!("Reconstructed: \"{}\"", reconstructed2);
    println!("Match original? {}\n", reconstructed2 == sentence2);
    
    // EXAMPLE 3: Question with multiple punctuation marks
    println!("📌 EXAMPLE 3: Questions and Exclamations");
    println!("─────────────────────────────────────\n");
    
    let sentence3 = "Really? Yes! Absolutely!!!";
    println!("Original sentence: \"{}\"", sentence3);
    
    let tokenizer3 = Tokenizer::new(sentence3.to_string());
    let tokens3 = tokenizer3.tokenize();
    println!("Tokens: {:?}", tokens3);
    
    let reconstructed3 = tokenizer3.detokenize(&tokens3);
    println!("Reconstructed: \"{}\"", reconstructed3);
    println!("Match original? {}\n", reconstructed3 == sentence3);
    
    // EXAMPLE 4: Token statistics
    println!("📌 EXAMPLE 4: Token Statistics");
    println!("─────────────────────────────────────\n");
    
    let sentence4 = "Tokenization is the process of breaking text into tokens.";
    println!("Original sentence: \"{}\"", sentence4);
    
    let tokenizer4 = Tokenizer::new(sentence4.to_string());
    let tokens4 = tokenizer4.tokenize();
    
    // Print token count
    println!("Total tokens: {}", tokens4.len());
    
    // Count only word tokens (excluding punctuation)
    // This shows how to filter tokens based on their type
    let word_tokens: Vec<_> = tokens4
        .iter()
        .filter(|t| !t.chars().all(|c| !c.is_alphanumeric())) // Keep tokens with letters/numbers
        .collect();
    
    println!("Word tokens: {}", word_tokens.len());
    println!("Punctuation tokens: {}", tokens4.len() - word_tokens.len());
    println!("Tokens breakdown: {:?}\n", tokens4);
    
    // EXAMPLE 5: Interactive-style demonstration
    println!("📌 EXAMPLE 5: Custom Sentence");
    println!("─────────────────────────────────────\n");
    
    let sentence5 = "This Rust project is awesome, isn't it? (Not really)";
    println!("Original sentence: \"{}\"", sentence5);
    
    let tokenizer5 = Tokenizer::new(sentence5.to_string());
    let tokens5 = tokenizer5.tokenize();
    
    println!("\nToken breakdown:");
    // Iterate through each token and show its index
    // This is useful for understanding token positions
    for (index, token) in tokens5.iter().enumerate() {
        // Determine if this is a word or punctuation
        let token_type = if token.chars().all(|c| !c.is_alphanumeric()) {
            "PUNCTUATION"
        } else {
            "WORD"
        };
        println!("  [{}] {} (type: {})", index, token, token_type);
    }
    
    let reconstructed5 = tokenizer5.detokenize(&tokens5);
    println!("\nReconstructed: \"{}\"", reconstructed5);
    println!("Perfect match? {}\n", reconstructed5 == sentence5);
    
    // EXAMPLE 6: Edge case - sentence with extra spaces
    println!("📌 EXAMPLE 6: Extra Spaces Handling");
    println!("─────────────────────────────────────\n");
    
    // Note: Multiple spaces get collapsed into single space during tokenization
    let sentence6 = "This  has   multiple    spaces.";
    println!("Original sentence: \"{}\"", sentence6);
    
    let tokenizer6 = Tokenizer::new(sentence6.to_string());
    let tokens6 = tokenizer6.tokenize();
    println!("Tokens: {:?}", tokens6);
    
    let reconstructed6 = tokenizer6.detokenize(&tokens6);
    println!("Reconstructed: \"{}\"", reconstructed6);
    println!("Note: Extra spaces are normalized to single spaces\n");
    
    // Print final summary
    println!("╔════════════════════════════════════════╗");
    println!("║        Tokenizer Examples Complete     ║");
    println!("╚════════════════════════════════════════╝");
}