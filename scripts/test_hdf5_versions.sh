#!/bin/bash
# Test the crate with different HDF5 versions installed in $HOME/opt

set -e

VERSIONS="1.10.11 1.12.3 1.13.3 1.14.5"
FAILED=""

for VERSION in $VERSIONS; do
    HDF5_DIR="$HOME/opt/hdf5-$VERSION"

    if [ ! -d "$HDF5_DIR" ]; then
        echo "HDF5 $VERSION not found at $HDF5_DIR, skipping..."
        continue
    fi

    echo "=========================================="
    echo "Testing with HDF5 $VERSION"
    echo "=========================================="

    # Reset and set library path for macOS (use only one version at a time)
    export DYLD_LIBRARY_PATH="$HDF5_DIR/lib"

    # Clean cargo cache to ensure fresh library linking
    cargo clean -p hdf5-rt 2>/dev/null || true

    # Run tests for each package separately to avoid macOS HDF5 cleanup issues
    if cargo test -p hdf5-rt 2>&1 | tee /dev/stderr | grep -q "test result: ok"; then
        echo "✓ HDF5 $VERSION: hdf5-rt OK"
    else
        echo "✗ HDF5 $VERSION: hdf5-rt FAILED"
        FAILED="$FAILED $VERSION"
    fi

    if cargo test -p hdf5-rt-types 2>&1 | tee /dev/stderr | grep -q "test result: ok"; then
        echo "✓ HDF5 $VERSION: hdf5-rt-types OK"
    else
        echo "✗ HDF5 $VERSION: hdf5-rt-types FAILED"
        FAILED="$FAILED $VERSION"
    fi

    echo ""
done

echo "=========================================="
if [ -z "$FAILED" ]; then
    echo "All tests completed successfully!"
else
    echo "Tests failed for versions:$FAILED"
    exit 1
fi
