#!/bin/bash
# Shell script to build all binaries and organize them

PROFILE="debug"
BUILD_CMD="cargo build"

# Parse arguments
if [ "$1" = "--release" ] || [ "$1" = "-r" ]; then
    PROFILE="release"
    BUILD_CMD="cargo build --release"
fi

echo "Building nullbox with profile: $PROFILE"
echo "========================================"

# Build
echo ""
echo "Step 1: Building all binaries..."
$BUILD_CMD

if [ $? -eq 0 ]; then
    echo ""
    echo "Step 2: Organizing binaries..."
    bash organize_bins.sh "$PROFILE"
    
    echo ""
    echo "========================================"
    echo "Build complete!"
    echo "Binaries location: target/$PROFILE/bin"
else
    echo ""
    echo "Build failed!"
    exit 1
fi
