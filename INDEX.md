# Index - Rust Multi-Target Build System

## Quick Navigation

### 🎯 Start Here
- **[START_HERE.md](START_HERE.md)** - Overview and navigation guide (2 min read)

### 📚 Documentation
- **[README.md](README.md)** - Complete guide to Cargo workspaces pattern (15 min)
- **[PHILOSOPHY.md](PHILOSOPHY.md)** - Why this approach is correct (10 min)
- **[QUICKSTART.md](QUICKSTART.md)** - 5-minute setup guide (5 min)
- **[CHANGES.md](CHANGES.md)** - What changed from tool approach (5 min)
- **[SOLUTION.md](SOLUTION.md)** - Auto-discovery & incremental builds (10 min)
- **[AUTO_DISCOVERY.md](AUTO_DISCOVERY.md)** - Detailed guide with examples (15 min)

### 💻 Example Project
- **[example/](example/)** - Working multi-crate Rust project
  - [example/README.md](example/README.md) - Example documentation
  - [example/AUTO_DISCOVERY.md](example/AUTO_DISCOVERY.md) - Auto-discovery guide
  - [example/Cargo.toml](example/Cargo.toml) - Workspace configuration
  - [example/update-workspace.sh](example/update-workspace.sh) - Auto-discovery script
  - `example/crates/` - Example crates:
    - `common/` - Base library (no dependencies)
    - `math/` - Math library (depends on common)
    - `utils/` - Utils library (depends on common)
    - `app/` - Binary application (depends on math and utils)

### 🔧 Scripts
- **[update-workspace.sh](update-workspace.sh)** - Auto-discover workspace members
- **[example/update-workspace.sh](example/update-workspace.sh)** - Auto-discover (in example)

---

## What Is This?

**NOT** a compiled tool or wrapper.

**BUT** a pattern for organizing Rust multi-target projects using Cargo workspaces:
- Workspace root `Cargo.toml` lists all member crates
- Each member crate declares dependencies in its `Cargo.toml`
- You run standard `cargo build` command
- Cargo handles everything: build order, parallelism, caching, etc.

---

## Reading Paths

### Path 1: Quick Overview (10 minutes)
1. This file (2 min)
2. [START_HERE.md](START_HERE.md) (2 min)
3. Try example: `cd example && cargo build && cargo run -p app` (3 min)
4. [example/README.md](example/README.md) (3 min)

### Path 2: Complete Understanding (40 minutes)
1. [START_HERE.md](START_HERE.md) (2 min)
2. [example/README.md](example/README.md) (5 min)
3. [QUICKSTART.md](QUICKSTART.md) (5 min)
4. [README.md](README.md) (15 min)
5. [PHILOSOPHY.md](PHILOSOPHY.md) (10 min)
6. [CHANGES.md](CHANGES.md) (5 min)

### Path 3: Build Your Own (15 minutes)
1. [QUICKSTART.md](QUICKSTART.md) - Follow the setup (5 min)
2. [README.md](README.md) - Reference section (10 min)
3. Create your project following the pattern

---

## Key Concepts

### 1. Workspace Structure
```
project/
├── Cargo.toml                    # Root: lists members
├── crates/
│   ├── core/
│   │   ├── Cargo.toml           # Declares dependencies
│   │   └── src/lib.rs
│   ├── utils/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   └── app/
│       ├── Cargo.toml
│       └── src/main.rs
```

### 2. Root Cargo.toml
```toml
[workspace]
members = ["crates/core", "crates/utils", "crates/app"]
```

### 3. Member Cargo.toml
```toml
[package]
name = "app"

[dependencies]
core = { path = "../core" }
utils = { path = "../utils" }
```

### 4. Build Command
```bash
cargo build              # Builds everything in dependency order
cargo build -p core     # Build specific crate
cargo test --workspace  # Test all crates
cargo tree              # Show dependency graph
```

---

## File Organization

```
/workspace/
├── INDEX.md              ← You are here
├── START_HERE.md         ← Best starting point
├── README.md             ← Complete reference
├── PHILOSOPHY.md         ← Why this approach
├── QUICKSTART.md         ← Setup guide
├── CHANGES.md            ← What changed
├── Cargo.toml            ← Root workspace
├── example/              ← Working example
│   ├── Cargo.toml
│   ├── README.md
│   ├── crates/
│   │   ├── common/       ← Base library
│   │   ├── math/         ← Depends on common
│   │   ├── utils/        ← Depends on common
│   │   └── app/          ← Depends on math, utils
│   └── target/
└── Dockerfile            ← Development environment
```

---

## Quick Start

### Try the Example (1 minute)
```bash
cd /workspace/example
cargo build
cargo run -p app
```

### Create Your Own (5 minutes)
```bash
mkdir my_project && cd my_project
mkdir -p crates/common crates/app

# Create root Cargo.toml
cat > Cargo.toml << 'EOF'
[workspace]
members = ["crates/common", "crates/app"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
EOF

# Create libraries and binaries
cargo new --lib crates/common
cargo new crates/app

# Add dependency to app
cd crates/app
cat >> Cargo.toml << 'EOF'

[dependencies]
common = { path = "../common" }
EOF

cd ../..
cargo build
```

---

## The Philosophy

**Use Rust's native toolchain properly, not build a wrapper around it.**

Just like:
- super_make doesn't replace Make
- It helps you organize projects so Make works better

This approach:
- Doesn't replace Cargo
- Shows you how to organize projects so Cargo works better

---

## Key Benefits

✅ No new tool to compile or install
✅ Uses standard Cargo commands everyone knows
✅ Automatic build order determination
✅ Parallel compilation by default
✅ Proper IDE integration (rust-analyzer)
✅ Standard across all Rust projects
✅ Zero maintenance burden

---

## Common Tasks

### Build Everything
```bash
cargo build
```

### Build Release Version
```bash
cargo build --release
```

### Test All Crates
```bash
cargo test --workspace
```

### See Dependency Graph
```bash
cargo tree
```

### Build Specific Crate
```bash
cargo build -p core
```

### Clean Everything
```bash
cargo clean
```

---

## Questions?

- **How do I...?** → [README.md](README.md)
- **Why does this work?** → [PHILOSOPHY.md](PHILOSOPHY.md)
- **Show me an example** → [example/README.md](example/README.md)
- **Quick setup** → [QUICKSTART.md](QUICKSTART.md)
- **Overview** → [START_HERE.md](START_HERE.md)

---

## Next Steps

1. ✅ You've read this index
2. → Read [START_HERE.md](START_HERE.md) next (2 min)
3. → Try the example (2 min)
4. → Read [README.md](README.md) for complete details (15 min)

**Total time to understand: ~30 minutes**

---

**Remember: This is about using Cargo properly. No special tool needed. Just good organization.**
