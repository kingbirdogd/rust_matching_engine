# Rust Multi-Target Build System - Simple Approach

## The Philosophy

**Just like super_make:**
1. Edit config file to list your components
2. Run the native build tool
3. Done!

No external scripts needed.

---

## For Rust with Cargo

### Step 1: Edit `Cargo.toml`

List your workspace members (just like Makefile lists targets):

```toml
[workspace]
members = [
    "crates/common",
    "crates/app",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
```

### Step 2: Run Cargo

```bash
cargo build
```

That's it! Everything builds automatically:
- Resolves dependencies
- Compiles in correct order
- Incremental compilation (only rebuilds what changed)

---

## Quick Example

```bash
# Create workspace
mkdir my_project && cd my_project

# Create Cargo.toml (list your crates)
cat > Cargo.toml << 'EOF'
[workspace]
members = ["crates/lib", "crates/app"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
EOF

# Create the crates
cargo new --lib crates/lib
cargo new crates/app

# Add dependency in crates/app/Cargo.toml
# [dependencies]
# lib = { path = "../lib" }

# Build everything
cargo build
```

---

## Common Commands

```bash
# Build everything
cargo build

# Build release (optimized)
cargo build --release

# Build specific crate
cargo build -p lib

# Test everything
cargo test --workspace

# See dependencies
cargo tree

# Clean
cargo clean
```

---

## Adding a New Crate

1. Create the crate: `cargo new --lib crates/newmodule`
2. Edit `Cargo.toml` - add `"crates/newmodule"` to members list
3. Build: `cargo build`

That's it.

---

## Incremental Builds

Cargo automatically does this - no configuration needed.

When you change one file:
- Only affected crates recompile
- Everything else uses cache
- 5-10x faster than clean builds

```bash
cargo build           # Compiles all
cargo build           # 0.00s - cached!
edit file
cargo build           # Only recompile what changed
```

---

## Comparison

| Task | super_make | Cargo |
|------|-----------|-------|
| List targets | Edit Makefile | Edit Cargo.toml |
| Build | `make` | `cargo build` |
| Build specific | `make target` | `cargo build -p crate` |
| Clean rebuild | `make clean` | `cargo clean` |
| Parallel | `make -j` | `cargo build -j` |
| Incremental | Make's logic | Cargo's logic |

**Same philosophy, different tool.**

---

## That's It!

No scripts, no special tools, no configuration beyond listing your crates.

Just:
1. List workspace members in `Cargo.toml`
2. Run `cargo build`
3. Cargo handles everything else

Exactly like super_make with `make`.
