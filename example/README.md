# Example: Multi-Crate Rust Project

This is a real example showing how to organize a multi-target Rust project using **Cargo workspaces** — the Rust equivalent of super_make's structure.

## Project Structure

```
example/
├── Cargo.toml                 # Root workspace config (like super_make's Makefile)
├── crates/
│   ├── common/                # Base library (no dependencies)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── math/                  # Math library (depends on common)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── utils/                 # Utils library (depends on common)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── app/                   # Binary app (depends on math + utils)
│       ├── Cargo.toml
│       └── src/main.rs
└── README.md                  # This file
```

**This mirrors super_make exactly:**
- Root Cargo.toml lists all workspace members (like super_make's Makefile)
- Each crate is self-contained with its own Cargo.toml
- Dependencies are declared in each crate's Cargo.toml (like including common.mk)

## Build Order

Cargo automatically determines:
1. common (no dependencies) → builds first
2. math (depends on common) → builds next
3. utils (depends on common) → builds next  
4. app (depends on math + utils) → builds last

You don't specify this manually — Cargo figures it out from the dependency graph.

## How to Use

### Build Everything

```bash
cd example
cargo build
```

Output:
```
Compiling common v0.1.0 (example/crates/common)
Compiling math v0.1.0 (example/crates/math)
Compiling utils v0.1.0 (example/crates/utils)
Compiling app v0.1.0 (example/crates/app)
Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

### Build Release Version

```bash
cargo build --release
```

Fully optimized binary in `target/release/app`

### Run the Application

```bash
cargo run -p app
```

Or directly:
```bash
./target/debug/app
```

Expected output:
```
[INFO] Starting application
-------------------
[INFO] add(10, 5)
Addition: 15
[INFO] multiply(3, 7)
Multiplication: 21
-------------------
[INFO] Application complete
```

### Test Everything

```bash
cargo test --workspace
```

Runs all tests in all crates in dependency order.

### Build Specific Crate

```bash
cargo build -p math
cargo build -p utils
cargo build -p app
```

### Show Dependency Tree

```bash
cargo tree
```

Output:
```
app v0.1.0 (example/crates/app)
├── common v0.1.0 (example/crates/common)
├── math v0.1.0 (example/crates/math)
│   └── common v0.1.0 (example/crates/common)
└── utils v0.1.0 (example/crates/utils)
    └── common v0.1.0 (example/crates/common)
```

### Clean Build Artifacts

```bash
cargo clean
```

## How This Compares to super_make

**super_make approach:**
```makefile
# Root Makefile (like our root Cargo.toml)
include common.mk
TARGETS = common math utils app

# crates/math/Makefile (like crates/math/Cargo.toml)
include $(PROJECT_HOME)/common.mk
DEPS = common

# Build command
make -j 4
```

**Our Rust approach:**
```toml
# Root Cargo.toml (like super_make's Makefile)
[workspace]
members = ["crates/common", "crates/math", "crates/utils", "crates/app"]

# crates/math/Cargo.toml (like including common.mk)
[dependencies]
common = { path = "../common" }

# Build command
cargo build -j 4
```

**Key point:** Instead of compiling a tool that uses Make and g++, we use Cargo directly. Same organizational philosophy, native toolchain.

## Configuration Details

### Root Cargo.toml

```toml
[workspace]
members = [                           # List all member crates
    "crates/common",
    "crates/math",
    "crates/utils",
    "crates/app",
]
resolver = "2"                        # Dependency resolver version

[workspace.package]                   # Shared version across all crates
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]
```

### Individual Crate (e.g., crates/app/Cargo.toml)

```toml
[package]
name = "app"
version.workspace = true              # Inherits from workspace
edition.workspace = true

[[bin]]                               # Define binary
name = "app"
path = "src/main.rs"

[dependencies]
common = { path = "../common" }       # Local path dependency
math = { path = "../math" }
utils = { path = "../utils" }
```

## What Gets Built

After `cargo build`:

```
target/
├── debug/
│   ├── deps/
│   │   ├── libcommon-*.rlib         # common library (internal)
│   │   ├── libmath-*.rlib           # math library (internal)
│   │   ├── libutils-*.rlib          # utils library (internal)
│   │   └── ...
│   └── app                          # Final binary (executable)
└── release/
    ├── deps/
    │   └── lib*.rlib
    └── app                          # Optimized binary
```

Only the final binary (`app`) is what you run. Library crates produce `.rlib` files (Rust library format) that other crates link against.

## Parallel Compilation

Cargo parallelizes automatically:

```bash
# Use 8 parallel jobs
cargo build -j 8

# Let Cargo decide (uses available CPU cores)
cargo build
```

No special flag needed — Cargo is smart about it.

## Modifying the Project

### Add a new crate

```bash
cargo new --lib crates/newlib
```

Then add to root Cargo.toml:
```toml
[workspace]
members = [
    "crates/common",
    "crates/newlib",    # ← Add here
    "crates/math",
    "crates/utils",
    "crates/app",
]
```

### Add a dependency between crates

Edit `crates/app/Cargo.toml`:
```toml
[dependencies]
newlib = { path = "../newlib" }       # ← Add here
math = { path = "../math" }
utils = { path = "../utils" }
```

Cargo will automatically recompile app and anything that depends on it.

### Check for errors (without building)

```bash
cargo check --workspace
```

Fast way to validate everything compiles before full build.

## Advantages of This Approach

✅ **No special tool** — Just use `cargo`
✅ **No Makefile syntax** — Simple TOML configuration
✅ **Automatic build order** — Cargo handles dependency resolution
✅ **Parallel by default** — Faster builds on multi-core systems
✅ **Automatic cycle detection** — Can't create invalid circular dependencies
✅ **IDE integration** — rust-analyzer understands this structure perfectly
✅ **CI/CD friendly** — Simple commands for any automation system
✅ **Same on all platforms** — Works identically on Linux, macOS, Windows

## Real-World Use Cases

This structure is used for:
- Monorepo projects with multiple binaries
- Frameworks with plugin systems
- Applications with modular architecture
- Libraries with companion tools

Examples:
- **Cargo itself** — Uses workspaces for multiple tools
- **Tokio async runtime** — Uses workspaces for modular libraries
- **Bevy game engine** — Uses workspaces for engine components

## Next Steps

1. **Explore the code** — Each crate is tiny and educational
2. **Modify and rebuild** — Edit `crates/app/src/main.rs` and `cargo run -p app`
3. **Add more crates** — Create `cargo new --lib crates/mynewlib`
4. **Scale up** — This pattern works for projects with 50+ crates

## Questions?

See [../README.md](../README.md) for detailed documentation.
