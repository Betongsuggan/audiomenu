#!/usr/bin/env bash
# Quick test script to verify device listing without launcher

set -euo pipefail

echo "Testing audiomenu device parsing..."
echo ""

# We'll create a simple test by temporarily modifying main.rs to just print devices
# For now, let's test it manually with the actual binary and Walker

echo "=== Testing Sink (Output) Devices ==="
echo "Running: audiomenu sink --launcher walker"
echo ""
echo "This will open Walker. Cancel it (ESC) to exit."
echo ""
read -p "Press Enter to test sink devices..."

cargo run --release -- sink --launcher walker || echo "User cancelled or no selection"

echo ""
echo ""
echo "=== Testing Source (Input) Devices ==="
echo "Running: audiomenu source --launcher walker"
echo ""
echo "This will open Walker. Cancel it (ESC) to exit."
echo ""
read -p "Press Enter to test source devices..."

cargo run --release -- source --launcher walker || echo "User cancelled or no selection"

echo ""
echo "Tests complete!"
