#!/bin/bash
# Auto-discover workspace members and update Cargo.toml
# This script finds all Cargo.toml files in subdirectories and updates the workspace members list

set -e

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Scanning for Cargo.toml files in $WORKSPACE_ROOT..."
echo ""

# Find all subdirectories with Cargo.toml (recursively, up to 2 levels)
CRATES=()
for dir in "$WORKSPACE_ROOT"/*/*; do
  if [ -f "$dir/Cargo.toml" ] 2>/dev/null; then
    # Get path relative to workspace root
    rel_path="${dir#$WORKSPACE_ROOT/}"
    CRATES+=("$rel_path")
    echo "  ✓ Found: $rel_path"
  fi
done

# Also check direct subdirectories
for dir in "$WORKSPACE_ROOT"/*; do
  if [ -f "$dir/Cargo.toml" ] && [ -d "$dir" ] && [ "$(basename "$dir")" != "target" ]; then
    rel_path="$(basename "$dir")"
    # Check if not already added
    if [[ ! " ${CRATES[@]} " =~ " ${rel_path} " ]]; then
      CRATES+=("$rel_path")
      echo "  ✓ Found: $rel_path"
    fi
  fi
done

echo ""
if [ ${#CRATES[@]} -eq 0 ]; then
  echo "No crates found! Exiting."
  exit 1
fi

# Sort crates
IFS=$'\n' CRATES=($(sort <<<"${CRATES[*]}"))
unset IFS

# Generate members list
MEMBERS_LIST=""
for i in "${!CRATES[@]}"; do
  if [ $i -lt $((${#CRATES[@]} - 1)) ]; then
    MEMBERS_LIST+="    \"${CRATES[$i]}\",
"
  else
    MEMBERS_LIST+="    \"${CRATES[$i]}\""
  fi
done

# Create new Cargo.toml
cat > "$WORKSPACE_ROOT/Cargo.toml" << EOF
[workspace]
members = [
$MEMBERS_LIST
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name"]
EOF

echo "✓ Generated Cargo.toml with ${#CRATES[@]} workspace members"
