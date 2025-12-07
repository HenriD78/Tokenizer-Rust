# 🦀 Rust Tokenizer Project

### [Link to my youtube presentation video](https://youtu.be/M5bc4fwzGWI?si=aAt-k23tL94EoLc1)

An initial beginer's approach to a tokenizer implementation in Rust that demonstrates fundamental natural language processing concepts. This project includes complete source code documentation, practical examples.

## Quick Start

### What Does It Do?

This tokenizer breaks down text into smaller, meaningful units called **tokens** and can reconstruct the original text from those tokens. It's perfect for learning how modern NLP systems (like BERT) process text.

**Example:**
```
Input:  "Hello, world! How are you?"
Tokens: ["Hello", ",", "world", "!", "How", "are", "you", "?"]
Output: "Hello, world! How are you?" ✅ (perfectly reconstructed)
```

### How It Works (Simple Overview)

1. **Tokenization**: The text is split by whitespace, then punctuation is separated from words
2. **Detokenization**: Tokens are intelligently joined back together using spacing rules (e.g., commas attach to previous words)
3. **Analysis**: Statistics about tokens (word count, punctuation count, average length) are computed
4. **Verification**: Compare original and reconstructed text to validate correctness

## Getting Started

### Prerequisites

- **Rust 1.56+** ([Install Rust](https://www.rust-lang.org/tools/install))
- No other dependencies required! This project uses only Rust's standard library.

### Installation & Setup

1. **Create a new Rust project:**
```bash
cargo new rust-tokenizer
cd rust-tokenizer
```

2. **Copy the source files:**
   - Replace `src/main.rs` with the provided `main.rs` file on github
   - Create `src/tokenizer.rs` file and copy the tokenizer implementation into it

3. **Run the project:**
Run the command into the project's directory
```bash
cargo run
```

4. **See all examples and output:**
The program will display 6 comprehensive examples showing all tokenizer features.

## Project Structure

```
rust-tokenizer/
├── Cargo.toml                 # Project configuration (no dependencies!)
├── README.md                  # This file
├── src/
    ├── main.rs                # 8 detailed examples with comments
    └── tokenizer.rs           # Core tokenizer implementation
└── ...
```

### Cargo.toml

Keep it simple—no external dependencies:

```toml
[package]
name = "rust-tokenizer"
version = "0.1.0"
edition = "2021"

[dependencies]
# No dependencies needed - pure Rust!
```

## Core Functionality

### The Tokenizer Struct

The `Tokenizer` type is the main abstraction:

```rust
pub struct Tokenizer {
    text: String,  // Stores the original text for comparison
}
```

### Main Functions

#### 1. `new(text: String) -> Self`
**Creates a new tokenizer instance**
```rust
let tokenizer = Tokenizer::new("Hello, world!".to_string());
```

#### 2. `tokenize(&self) -> Vec<String>`
**Breaks text into tokens**
```rust
let tokens = tokenizer.tokenize();
// Result: ["Hello", ",", "world", "!"]
```

How it works:
- Splits by whitespace
- Separates punctuation from words
- Preserves apostrophes and hyphens in words ("don't" stays as one token)

#### 3. `detokenize(&self, tokens: &[String]) -> String`
**Reconstructs text from tokens with intelligent spacing**
```rust
let reconstructed = tokenizer.detokenize(&tokens);
// Result: "Hello, world!"
```

Smart spacing rules:
- Punctuation like `.`, `,`, `!`, `?` attaches to previous word (no space before)
- Opening brackets `(`, `[`, `{` attach to next word (no space after)
- Other tokens separated by spaces

#### 4. `original_text(&self) -> &str`
**Returns the stored original text**
```rust
let original = tokenizer.original_text();
```

Useful for:
- Comparing original with reconstructed versions
- Validating tokenization correctness
- Verifying reversibility

#### 5. `analyze_tokens(&self, tokens: &[String]) -> (usize, usize, usize, f64)`
**Computes token statistics**
```rust
let (total, words, punctuation, avg_length) = tokenizer.analyze_tokens(&tokens);
```

Returns:
- `total`: Total number of tokens
- `words`: Count of word tokens (containing alphanumeric characters)
- `punctuation`: Count of punctuation tokens
- `avg_length`: Average characters per token

## Running the Project

When you execute `cargo run`, you'll see 6 examples:

```
╔════════════════════════════════════════╗
║     Rust Tokenizer - Main Program      ║
╚════════════════════════════════════════╝

📌 EXAMPLE 1: Simple Tokenization
─────────────────────────────────────

Original sentence: "Hello, world! How are you?"
Tokens: ["Hello", ",", "world", "!", "How", "are", "you", "?"]
Reconstructed: "Hello, world! How are you?"
Match original? true

[... 5 more examples follow ...]
```

Each example demonstrates specific features with detailed output and explanations.

## How This Tokenizer Differs from Professional Systems

| Feature | This Project | BERT/GPT |
|---------|------------|---------|
| **Tokenization level** | Word-level + punctuation | Subword (BPE/WordPiece) |
| **Vocabulary** | Unlimited (rules-based) | Fixed set (30K-100K tokens) |
| **Unknown words** | Preserved as-is | Decomposed into subwords |
| **Reversibility** | Perfect reconstruction | Some information loss |
| **Language support** | English-focused | Multilingual variants |
| **Production ready** | Educational only | Yes |
| **Dependencies** | None (std lib only) | Multiple (transformers, torch, etc.) |

Professional NLP systems use more sophisticated approaches because they need to:
- Handle rare and unknown words efficiently
- Compress vocabulary size for memory efficiency
- Balance semantic meaning with model performance
- Support multiple languages with different linguistic properties


## Learning Resources

- **Rust Language**: https://doc.rust-lang.org/book/
- **NLP Fundamentals**: NLTK Book (https://www.nltk.org/book/)
- **BERT Paper**: https://arxiv.org/abs/1810.04805
- **Tokenization Techniques**: https://huggingface.co/docs/transformers/tokenizer_summary
