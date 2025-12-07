/// The Tokenizer struct is responsible for breaking text into tokens
/// and reconstructing text from tokens.
/// A token is a meaningful unit of text - typically a word or punctuation mark
pub struct Tokenizer {
    /// The original text that will be tokenized
    /// We store this to preserve the exact original for comparison, to make sure we got the tokenizer process right
    text: String,
}

impl Tokenizer {
    /// Creates a new Tokenizer instance with the given text
    /// 
    /// # Arguments
    /// * `text` - The text string to tokenize (takes ownership)
    /// 
    /// # Returns
    /// A new Tokenizer instance ready to process the text
    /// 
    /// # Example
    /// ```
    /// let tokenizer = Tokenizer::new("Hello, world!".to_string());
    /// ```
    pub fn new(text: String) -> Self {
        Tokenizer { text }
    }
    
    /// Tokenizes the stored text into a vector of token strings
    /// 
    /// This function:
    /// 1. Splits the text by whitespace to get word-like units
    /// 2. Separates punctuation from words
    /// 3. Returns all tokens as a Vec<String>
    /// 
    /// # Returns
    /// A vector of tokens (words and punctuation as separate entries)
    /// 
    /// # Example
    /// ```
    /// let tokenizer = Tokenizer::new("Hello, world!".to_string());
    /// let tokens = tokenizer.tokenize();
    /// // tokens will be: ["Hello", ",", "world", "!"]
    /// ```
    pub fn tokenize(&self) -> Vec<String> {
        // Create a mutable vector to store our tokens
        let mut tokens: Vec<String> = Vec::new();
        
        // Split the text by whitespace using split_whitespace()
        // This handles multiple spaces, tabs, newlines, etc. automatically
        for word_unit in self.text.split_whitespace() {
            // For each "word" (which might contain punctuation), we need to separate
            // punctuation from the actual word characters
            
            // We'll process the word character by character
            let mut current_token = String::new();
            
            for character in word_unit.chars() {
                // Check if this character is alphanumeric (letter or digit)
                if character.is_alphanumeric() || character == '\'' || character == '-' {
                    // These characters are part of words, so add them to current token
                    // (apostrophes and hyphens are often part of words like "don't" or "mother-in-law")
                    current_token.push(character);
                } else {
                    // This character is punctuation
                    // First, if we've been building a word token, save it
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    
                    // Then save the punctuation as its own token
                    tokens.push(character.to_string());
                }
            }
            
            // After processing all characters in this word unit,
            // if there's still a token being built, save it
            if !current_token.is_empty() {
                tokens.push(current_token);
            }
        }
        
        // Return the complete list of tokens
        tokens
    }
    
    /// Reconstructs the original text from a list of tokens
    /// 
    /// This function uses intelligent spacing rules:
    /// - Punctuation like . , ! ? ; : ) ] } gets NO space before it
    /// - Opening brackets ( [ { get NO space after them
    /// - Other words are separated by spaces
    /// 
    /// # Arguments
    /// * `tokens` - A vector of token strings to recombine
    /// 
    /// # Returns
    /// A reconstructed string from the tokens
    /// 
    /// # Example
    /// ```
    /// let tokens = vec!["Hello", ",", "world", "!"];
    /// let reconstructed = tokenizer.detokenize(&tokens);
    /// // Result: "Hello, world!"
    /// ```
    pub fn detokenize(&self, tokens: &[String]) -> String {
        // If there are no tokens, return an empty string
        if tokens.is_empty() {
            return String::new();
        }
        
        // Create a string to build the result
        let mut result = String::new();
        
        // Iterate through each token with its index
        for (index, token) in tokens.iter().enumerate() {
            // On the first token, just add it without any space
            if index == 0 {
                result.push_str(token);
            } else {
                // For tokens after the first, we need to decide about spacing
                
                // These characters should NOT have a space before them
                // because they attach to the previous word
                let no_space_before = ['.', ',', '!', '?', ';', ':', ')', ']', '}', '"'];
                
                // These characters should NOT have a space after them
                // because the next word attaches to them
                let no_space_after = ['(', '[', '{', '"'];
                
                // Check if the current token starts with a no-space character
                let first_char = token.chars().next().unwrap_or(' ');
                let should_add_space = !no_space_before.contains(&first_char);
                
                // Also check if the previous token is a no-space-after character
                let prev_token = &tokens[index - 1];
                let prev_last_char = prev_token.chars().last().unwrap_or(' ');
                let prev_allows_space = !no_space_after.contains(&prev_last_char);
                
                // Add space only if both conditions are met
                if should_add_space && prev_allows_space {
                    result.push(' ');
                }
                
                // Add the current token to the result
                result.push_str(token);
            }
        }
        
        // Return the reconstructed text
        result
    }
    
    /// Returns the original text that was stored in this Tokenizer
    /// 
    /// # Returns
    /// A reference to the original text string
    pub fn original_text(&self) -> &str {
        &self.text
    }
    
    /// Analyzes and returns statistics about the tokens
    /// 
    /// # Arguments
    /// * `tokens` - A vector of tokens to analyze
    /// 
    /// # Returns
    /// A tuple containing:
    /// - Total number of tokens
    /// - Number of word tokens (containing alphanumeric characters)
    /// - Number of punctuation tokens
    /// - Average token length
    pub fn analyze_tokens(&self, tokens: &[String]) -> (usize, usize, usize, f64) {
        let total = tokens.len();
        
        // Count words by checking if token contains alphanumeric characters
        let words = tokens
            .iter()
            .filter(|t| t.chars().any(|c| c.is_alphanumeric()))
            .count();
        
        // Punctuation tokens are everything else
        let punctuation = total - words;
        
        // Calculate average token length
        let total_chars: usize = tokens.iter().map(|t| t.len()).sum();
        let avg_length = if total > 0 {
            total_chars as f64 / total as f64
        } else {
            0.0
        };
        
        (total, words, punctuation, avg_length)
    }
} 

