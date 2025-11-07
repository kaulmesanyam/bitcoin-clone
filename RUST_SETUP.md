# Rust Setup Guide for Bitcoin Clone

## Installing Rust

### macOS/Linux:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows:
Download and run: https://rustup.rs/

After installation, restart your terminal or run:
```bash
source $HOME/.cargo/env
```

## Verify Installation

```bash
rustc --version
cargo --version
```

## Creating Your First Rust Project

```bash
# Create a new Rust project
cargo new bitcoin-clone
cd bitcoin-clone

# Build the project
cargo build

# Run the project
cargo run

# Run tests
cargo test
```

## Project Structure

```
bitcoin-clone/
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock          # Locked dependency versions (auto-generated)
├── src/
│   ├── main.rs         # Entry point
│   └── lib.rs          # Library code (if making a library)
└── target/             # Build artifacts (gitignored)
```

## Essential Rust Concepts for Phase 1

### 1. Variables and Mutability
```rust
let x = 5;        // Immutable
let mut y = 5;    // Mutable
```

### 2. Structs (like classes)
```rust
struct Block {
    index: u64,
    previous_hash: String,
    data: String,
}
```

### 3. Implementation Blocks
```rust
impl Block {
    fn new(index: u64, previous_hash: String, data: String) -> Self {
        Block { index, previous_hash, data }
    }
    
    fn calculate_hash(&self) -> String {
        // Hash calculation
    }
}
```

### 4. Ownership & Borrowing
- **Ownership**: Each value has one owner
- **Borrowing**: `&` creates a reference (borrow)
- **Move**: Values are moved, not copied (unless they implement `Copy`)

### 5. Vectors (dynamic arrays)
```rust
let mut blocks: Vec<Block> = Vec::new();
blocks.push(block);
```

### 6. String Types
- `String`: Owned, growable string
- `&str`: String slice (reference)

## Useful Crates for Phase 1

Add to `Cargo.toml`:

```toml
[dependencies]
sha2 = "0.10"      # SHA-256 hashing
hex = "0.4"        # Hex encoding/decoding
serde = { version = "1.0", features = ["derive"] }  # Serialization
serde_json = "1.0" # JSON serialization
```

## Common Commands

```bash
# Add a dependency
cargo add sha2

# Update dependencies
cargo update

# Check code (without building)
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Build in release mode (optimized)
cargo build --release
```

## Learning Path

1. **Start with**: The Rust Book chapters 1-6 (ownership, structs, enums)
2. **Then**: Rustlings exercises (hands-on practice)
3. **For this project**: Learn as you build!

## Helpful Resources

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings
- **Rust API Docs**: https://doc.rust-lang.org/std/

## Tips

1. **Read compiler errors carefully** - Rust's compiler is excellent and teaches you!
2. **Start simple** - Don't try to use advanced features immediately
3. **Use `cargo check` frequently** - Fast feedback without full compilation
4. **Embrace the borrow checker** - It's preventing bugs, not being annoying!

## Next Steps

Once Rust is installed, you're ready for Phase 1! We'll start with:
- Creating a Block struct
- Implementing hash calculation
- Linking blocks together

