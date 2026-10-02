# Rust Multi-Target Build System

A simple, pattern-based approach to organizing complex Rust projects using **Cargo workspaces** — directly analogous to super_make's organization of C++ projects.

## Philosophy

Just as super_make uses:
- `common.mk` → shared build configuration
- `Makefile` → project setup  
- `make` command → executes the build

This system uses:
- `Cargo.toml` (workspace root) → shared configuration and workspace members
- `Cargo.toml` (per crate) → individual crate setup
- `cargo` command → executes the build

**No compiled tool. No wrapper. Just Cargo and configuration.**

## Project Structure

Mirrors super_make's simplicity:

```
project/
├── Cargo.toml                 # Workspace config (like super_make root Makefile)
├── crates/
│   ├── common/                # Shared library
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── lib_a/                 # Library A (depends on common)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── lib_b/                 # Library B (depends on common)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── app/                   # Binary (depends on lib_a, lib_b)
│       ├── Cargo.toml
│       └── src/main.rs
└── build.sh                   # Optional: orchestration script
```

This is exactly like super_make:
- One root config (Cargo.toml vs Makefile)
- Crates scattered in directories (like super_make's subdirectories)
- Each crate self-contained (each includes common config)
- Dependencies declared simply (Cargo.toml dependencies vs Makefile DEPS)

## Cargo Workspace Basics

### Root Workspace Config

`Cargo.toml` at project root:

```toml
[workspace]
members = [
    "crates/common",
    "crates/lib_a",
    "crates/lib_b", 
    "crates/app",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
```

**This is exactly like super_make's Makefile:**
- super_make: Edit `Makefile` to list targets → Run `make`
- Cargo: Edit `Cargo.toml` to list members → Run `cargo build`

Just declare your workspace members, then use the native tool. No scripts needed.

### Individual Crate Config

Each crate has its own `Cargo.toml` (like each subdirectory includes common.mk):

`crates/common/Cargo.toml`:
```toml
[package]
name = "common"
version.workspace = true
edition.workspace = true

[lib]
```

`crates/lib_a/Cargo.toml`:
```toml
[package]
name = "lib_a"
version.workspace = true
edition.workspace = true

[dependencies]
common = { path = "../common" }
```

`crates/app/Cargo.toml`:
```toml
[package]
name = "app"
version.workspace = true
edition.workspace = true

[[bin]]
name = "app"
path = "src/main.rs"

[dependencies]
lib_a = { path = "../lib_a" }
lib_b = { path = "../lib_b" }
```

## Build Commands

Just use `cargo` directly — **no rust_make tool needed**:

```bash
# Build everything
cargo build

# Build release
cargo build --release

# Build specific crate
cargo build -p common
cargo build -p lib_a
cargo build -p app

# Build with all dependencies resolved
cargo build --workspace

# Test everything
cargo test --workspace

# Clean everything
cargo clean

# Show dependency tree
cargo tree

# Check for errors
cargo check --workspace
```

This is **functionally identical to super_make** but using Cargo instead of Make.

## Multi-Target Output

After `cargo build`:

```
target/
├── debug/
│   └── app                    # Binary executable
└── release/
    └── app                    # Optimized binary
```

Library artifacts are in the build cache automatically.

## Parallel Builds

Cargo handles parallelism automatically:

```bash
# Use all available cores
cargo build -j 8

# Let cargo decide
cargo build  # auto-parallelized
```

No need for Make's `-j` flag — Cargo does this intelligently.

## Build Order & Dependency Resolution

Just declare dependencies in `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }    # common builds first
lib_a = { path = "../lib_a" }      # then lib_a
lib_b = { path = "../lib_b" }      # then lib_b
```

Cargo automatically:
- Resolves build order
- Detects cycles
- Reports errors
- Caches correctly

**No topological sort needed** — Cargo handles it natively.

## Optional: Build Orchestration Script

If you want a simple wrapper (like super_make's root Makefile), use a shell script:

`build.sh`:
```bash
#!/bin/bash
set -e

echo "Building workspace..."
cargo build --release

echo "Running tests..."
cargo test --workspace

echo "Build complete!"
ls -lh target/release/app
```

Run it:
```bash
chmod +x build.sh
./build.sh
```

But this is **optional** — `cargo build` does everything.

## Complex Project Example

Multi-level dependencies (like super_make projects):

```
crates/
├── common/                    # Base library (no deps)
│   └── Cargo.toml
├── utils/                     # Utils (depends on common)
│   ├── Cargo.toml
│   └── [dependencies]
│       common = { path = "../common" }
├── database/                  # DB layer (depends on common, utils)
│   ├── Cargo.toml
│   └── [dependencies]
│       common = { path = "../common" }
│       utils = { path = "../utils" }
├── api/                       # API (depends on database)
│   ├── Cargo.toml
│   └── [dependencies]
│       database = { path = "../database" }
└── cli/                       # CLI app (depends on api)
    ├── Cargo.toml
    └── [dependencies]
        api = { path = "../api" }
```

Build order **automatically determined** by Cargo:
1. common (no dependencies)
2. utils (depends on common)
3. database (depends on common, utils)
4. api (depends on database)
5. cli (depends on api)

No manual ordering needed.

## Features & Configuration

Use Cargo features (Rust native):

```toml
[package]
name = "lib_a"
version.workspace = true
edition.workspace = true

[features]
default = ["utils"]
full = ["utils", "logging", "metrics"]

[dependencies]
common = { path = "../common" }
logging = { version = "0.4", optional = true }
metrics = { version = "0.20", optional = true }
```

Build with features:
```bash
cargo build --features full
cargo build -p lib_a --features full
```

## Testing Across Workspace

Run all tests in build order:

```bash
cargo test --workspace              # All tests
cargo test --workspace --release    # Optimized test build
cargo test -p lib_a                 # Test one crate
```

## Comparison: super_make vs This Pattern

| Aspect | super_make | This Rust Pattern |
|--------|-----------|-------------------|
| Root config | Makefile | Cargo.toml (workspace) |
| Subproject config | Makefile including common.mk | Cargo.toml with [dependencies] |
| Build tool | make + g++ | cargo (which uses rustc) |
| Invoke build | make -j 4 | cargo build -j 4 |
| Dependency resolution | Manual (DEPS variable) | Automatic (path dependencies) |
| Cycle detection | Manual inspection | Automatic error |
| Build order | Manual (Make determines) | Automatic (Cargo determines) |
| Parallel execution | make's parallelism | Cargo's parallelism |
| Test orchestration | make test | cargo test --workspace |
| Caching | Make's rebuild logic | Cargo's dependency caching |

## Key Differences (Advantages)

✅ **Simpler configuration** - TOML vs Make syntax
✅ **Automatic cycle detection** - No invalid dependency graphs
✅ **Better parallelism** - Cargo understands Rust better than Make
✅ **Integrated testing** - cargo test works across workspace
✅ **Native feature support** - Cargo features, not preprocessor
✅ **Portable** - Same on Linux, macOS, Windows
✅ **IDE integration** - rust-analyzer understands workspaces

## When to Use This Pattern

✅ Multiple Rust crates in one project
✅ Shared libraries + binaries
✅ Complex internal dependencies
✅ Monorepo structure
✅ Team developing multiple components

## When NOT to Use

❌ Single binary/library → just use `cargo new`
❌ External dependencies only → plain cargo.toml works
❌ C/C++ compilation → need actual build system

## Setup: From Scratch

```bash
# Create workspace
mkdir my_project && cd my_project

# Create root Cargo.toml with workspace config
cat > Cargo.toml << 'EOF'
[workspace]
members = ["crates/common", "crates/app"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
EOF

# Create common library
cargo new --lib crates/common

# Create app binary
cargo new crates/app

# Add dependency: app -> common
cd crates/app
cat >> Cargo.toml << 'EOF'

[dependencies]
common = { path = "../common" }
EOF

cd ../..

# Build everything
cargo build

# Done!
```

That's it. No scripts. Just standard Cargo workflow.

## For CI/CD

Just use cargo in your pipeline:

```yaml
# GitHub Actions example
- name: Build
  run: cargo build --workspace --release

- name: Test
  run: cargo test --workspace

- name: Check
  run: cargo check --workspace --all-targets
```

No special tooling needed.

## Documentation per Crate

Each crate can have its own `README.md`:

```
crates/
├── common/
│   ├── Cargo.toml
│   ├── README.md          # Explains common library
│   └── src/
├── lib_a/
│   ├── Cargo.toml
│   ├── README.md          # Explains lib_a
│   └── src/
└── app/
    ├── Cargo.toml
    ├── README.md          # Explains app and how to use it
    └── src/
```

Build HTML docs:
```bash
cargo doc --workspace --open
```

## Performance Considerations

**Cargo's incremental compilation:**
- Changes to common → rebuilds common, lib_a, lib_b, app
- Changes to lib_a only → rebuilds lib_a and app (not common)
- Minimal recompilation (better than Make)

**Release builds:**
```bash
cargo build --release --workspace -j 8
```

Fully optimized, parallel compilation across crates.

## Dependency Graph Visualization

Cargo provides this natively:

```bash
# Install tool
cargo install cargo-tree

# View dependency tree
cargo tree

# Output:
# my_project v0.1.0 (/path/to/project)
# ├── app v0.1.0 (/path/to/crates/app)
# │   ├── lib_a v0.1.0 (/path/to/crates/lib_a)
# │   │   └── common v0.1.0 (/path/to/crates/common)
# │   └── lib_b v0.1.0 (/path/to/crates/lib_b)
# │       └── common v0.1.0 (/path/to/crates/common)
# └── ...
```

Or use `cargo metadata` for programmatic access.

## Summary

**This is super_make for Rust:**

| Element | super_make | Rust Pattern |
|---------|-----------|--------------|
| Unites subprojects | root Makefile | root Cargo.toml |
| Each subproject config | Makefile + include | Cargo.toml with [dependencies] |
| Orchestration | make command | cargo command |
| Compiler/toolchain | g++, ar, gcc | rustc (via cargo) |
| No special tool | ✓ Uses make | ✓ Uses cargo |

**You don't build a tool. You use the native toolchain directly with proper project organization.**

---

Start with the example structure below or dive into the documentation.
