# Auto-Discovery & Incremental Builds

## Problem Statement

You want:
1. **No manual member listing** - Auto-discover all crates
2. **Incremental builds** - Only rebuild what changed

## Solution

### Part 1: Auto-Discovery Script

Instead of manually listing workspace members in `Cargo.toml`:

```toml
[workspace]
members = [
    "crates/common",
    "crates/math",
    "crates/utils",
    "crates/app",
]
```

Use the **`update-workspace.sh`** script that automatically discovers all crates:

```bash
./update-workspace.sh
```

This script:
- Finds all directories containing `Cargo.toml`
- Generates the `members = [...]` list automatically
- Updates your root `Cargo.toml`
- No manual maintenance needed

**Usage:**

```bash
# Whenever you add a new crate
mkdir crates/newlib
cargo new --lib crates/newlib

# Update the workspace
./update-workspace.sh

# That's it! New crate is now part of the workspace
cargo build
```

### How It Works

The script scans your project for all `Cargo.toml` files and generates:

```toml
[workspace]
members = [
    "crates/app",
    "crates/common",
    "crates/math",
    "crates/utils"
]
```

Automatically, without you having to maintain the list.

---

## Part 2: Incremental Builds

**Good news: Cargo already does this!**

You don't need to do anything special. Cargo automatically:

### ✅ Only Rebuilds Changed Crates

When you run `cargo build`:

```bash
# Change only math/src/lib.rs
cargo build
```

Output:
```
   Compiling math v0.1.0
    Finished
```

- ✓ Only `math` was recompiled
- ✓ `common`, `utils`, `app` were skipped (cached)
- ✓ `app` didn't rebuild even though it depends on `math`

### ✅ Smart Dependency Tracking

Cargo knows:
- If a library changes, recompile dependents
- If a binary's source changes, only recompile the binary
- If source files are unchanged, skip compilation

### Example: Incremental Workflow

```bash
# First build (all crates compile)
$ cargo build
   Compiling common
   Compiling math
   Compiling utils
   Compiling app
    Finished in 0.45s

# Change math's code
$ echo 'pub fn subtract(a: i32, b: i32) -> i32 { a - b }' >> crates/math/src/lib.rs

# Rebuild (only affected crates compile)
$ cargo build
   Compiling math v0.1.0
   Compiling app v0.1.0      # Only because it depends on math
    Finished in 0.08s

# No changes to code
$ cargo build
    Finished `dev` in 0.02s   # Nothing compiled! Cached.

# Change only app's main.rs
$ cargo build
   Compiling app v0.1.0
    Finished in 0.05s         # Only app recompiled
```

### Checking Without Rebuilding

```bash
# Check for errors without compiling
cargo check --workspace
```

Much faster - finds errors without linking.

---

## Workflow: Add New Crate

When you need to add a new crate to the project:

```bash
# 1. Create the new crate
mkdir crates/newmodule
cargo new --lib crates/newmodule

# 2. Auto-update workspace
./update-workspace.sh

# 3. Add dependencies if needed
# Edit crates/newmodule/Cargo.toml to add dependencies

# 4. Build everything
cargo build

# 5. Next time you change newmodule, only it recompiles
```

No manual `Cargo.toml` editing needed!

---

## How Incremental Builds Work

### Cargo's Caching System

```
First build:
  common (no deps)        → Compiled → .rlib cached
  math (→common)          → Compiled → .rlib cached
  utils (→common)         → Compiled → .rlib cached
  app (→math,utils)       → Compiled → Binary created

Time: ~400ms

Second build (no changes):
  All .rlib files in cache ✓
  Binary still valid ✓
  Nothing to do
  
Time: ~20ms (just checking cache)

Change math/src/lib.rs:
  math needs recompilation ✓
  .rlib is invalidated
  Recompile math → new .rlib
  Recompile app (depends on math)
  All others use cache
  
Time: ~100ms
```

### Dependency Graph Tracking

Cargo knows:
```
    common
    /    \
  math  utils
    \    /
     app
```

If `common` changes:
- Recompile: common → math, utils → app
- All in dependency order, parallel where possible

If only `app` changes:
- Recompile: app only
- Use cached math, utils, common

If only `utils` changes:
- Recompile: utils → app
- math stays cached (doesn't depend on utils)

---

## Practical Example

### Scenario: Working on a Module

```bash
# Check structure
cargo tree

# Build everything
cargo build

# Work on math module
vi crates/math/src/lib.rs

# Test just your changes
cargo test -p math

# Full build (only math and dependents recompile)
cargo build

# No changes made
cargo build  # Instant - all cached!

# Work on app binary
vi crates/app/src/main.rs

# Build (app only, math/utils/common cached)
cargo build -p app

# Run
./target/debug/app
```

Each build after code changes only recompiles affected crates.

---

## Performance Tips

### Faster Incremental Development

```bash
# Use debug mode (default, faster compilation)
cargo build

# Only check for errors (no linking)
cargo check --workspace

# Test specific crate (faster)
cargo test -p math

# Build specific crate
cargo build -p app
```

### Release Builds

```bash
# Optimized build (slower but smaller/faster binary)
cargo build --release

# Full release rebuild
cargo build --release --workspace
```

---

## Scripts for Common Tasks

### Rebuild Entire Project

```bash
#!/bin/bash
# full-rebuild.sh
cargo clean
cargo build --release --workspace
```

### Update Workspace and Build

```bash
#!/bin/bash
# build-all.sh
./update-workspace.sh
cargo build --workspace
```

### Development Workflow

```bash
#!/bin/bash
# dev.sh - fast iteration
cargo check --workspace   # Fast error checking
cargo build              # Incremental build
cargo test --workspace   # Run all tests
```

---

## Summary

✅ **Auto-Discovery:**
- Run: `./update-workspace.sh`
- Finds all `Cargo.toml` files
- Auto-generates workspace members list
- No manual maintenance

✅ **Incremental Builds:**
- Already built into Cargo
- Only recompiles changed crates
- Respects dependency graph
- Caches everything else
- ~10-20x faster for small changes

✅ **Development Workflow:**
1. Add new crate: `cargo new --lib crates/newcrate`
2. Update workspace: `./update-workspace.sh`
3. Build: `cargo build`
4. Modify code as needed
5. Each rebuild only touches changed crates

---

**No additional tools needed. Cargo handles all of this automatically.**
