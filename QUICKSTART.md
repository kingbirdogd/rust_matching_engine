# Quick Start - Rust Multi-Target Projects

Get a multi-target Rust project working in 5 minutes.

## What You're Building

A project with:
- Multiple Rust crates (libraries + binary)
- Clear dependency structure
- Single build command: `cargo build`
- No special tools or wrappers

## Step 1: Create the Workspace

```bash
mkdir my_project
cd my_project

cat > Cargo.toml << 'EOF'
[workspace]
members = ["crates/core", "crates/utils", "crates/app"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
EOF
```

## Step 2: Create Crates

```bash
# Create library crates
cargo new --lib crates/core
cargo new --lib crates/utils

# Create binary crate
cargo new crates/app
```

## Step 3: Declare Dependencies

**crates/utils/Cargo.toml** - depends on core:
```toml
[package]
name = "utils"
version.workspace = true
edition.workspace = true

[dependencies]
core = { path = "../core" }
```

**crates/app/Cargo.toml** - depends on both:
```toml
[package]
name = "app"
version.workspace = true
edition.workspace = true

[[bin]]
name = "app"
path = "src/main.rs"

[dependencies]
core = { path = "../core" }
utils = { path = "../utils" }
```

## Step 4: Add Code

**crates/core/src/lib.rs:**
```rust
pub fn hello() {
    println!("Hello from core!");
}
```

**crates/utils/src/lib.rs:**
```rust
use core;

pub fn say_hello_twice() {
    core::hello();
    core::hello();
}
```

**crates/app/src/main.rs:**
```rust
use core;
use utils;

fn main() {
    core::hello();
    utils::say_hello_twice();
}
```

## Step 5: Build & Run

```bash
# Build everything
cargo build

# Run the app
cargo run -p app

# Or run binary directly
./target/debug/app
```

Output:
```
Hello from core!
Hello from core!
Hello from core!
```

## That's It! 

You now have a multi-target project that automatically:
- Resolves build order
- Compiles in parallel
- Detects circular dependencies
- Caches correctly
- Tests across all crates

## Common Commands

```bash
# Build everything
cargo build

# Build release (optimized)
cargo build --release

# Run tests in all crates
cargo test --workspace

# Check for errors (fast)
cargo check --workspace

# See dependency tree
cargo tree

# Build specific crate
cargo build -p core
cargo build -p app

# Clean everything
cargo clean
```

## Next Steps

1. **Learn more**: See [README.md](README.md)
2. **See example**: Check [example/](example/) for a real project
3. **Understand philosophy**: Read [PHILOSOPHY.md](PHILOSOPHY.md)
4. **Explore structure**: Look at workspace organization in [example/Cargo.toml](example/Cargo.toml)

## Common Issues

### "error[E0433]: cannot find crate `core`"
Make sure the dependency is declared in Cargo.toml:
```toml
[dependencies]
core = { path = "../core" }
```

### Build fails with "unresolved import"
Check that:
1. The crate is in workspace members
2. The dependency is declared in Cargo.toml
3. You ran `cargo build` from the workspace root

### "could not compile `app`"
Try:
```bash
cargo clean
cargo build
```

## Project Layout Checklist

✓ Root Cargo.toml with [workspace] section
✓ Each crate in crates/ directory
✓ Each crate has its own Cargo.toml
✓ Dependencies declared as `{ path = "../..." }`
✓ Libraries are `cargo new --lib`
✓ Binaries are `cargo new`

## Performance Tips

- Use `cargo build --release` for optimized binaries
- Use `cargo check` to quickly validate (no linking)
- Use `cargo build -p NAME` to build one crate only
- Use `-j N` to limit parallel jobs: `cargo build -j 2`

## More Examples

See [example/](example/) for:
- Working multi-crate project
- Test examples
- Common patterns
- Real dependency structure

---

**That's the quickstart. For deep dive, read [README.md](README.md).**
