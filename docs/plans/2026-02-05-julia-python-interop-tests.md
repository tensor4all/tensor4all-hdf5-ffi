# Julia/Python Interoperability Tests Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Test that Rust runtime-loading feature can use HDF5 libraries from Julia (HDF5_jll) and Python (h5py), and read/write files created by these languages.

**Architecture:** Create a Rust CLI binary that accepts HDF5 library path and test file path as arguments. Julia/Python scripts create test files, invoke the Rust binary with their HDF5 library paths, and verify results.

**Tech Stack:** Rust (clap for CLI), Julia (HDF5.jl, HDF5_jll), Python (h5py, numpy)

---

## Task 1: Create Rust Interop Test Binary

**Files:**
- Create: `hdf5/examples/interop_test.rs`
- Modify: `hdf5/Cargo.toml` (add clap dependency)

**Step 1: Add clap dependency to Cargo.toml**

In `hdf5/Cargo.toml`, add to `[dev-dependencies]`:

```toml
clap = { version = "4", features = ["derive"] }
```

**Step 2: Create the interop test binary**

Create `hdf5/examples/interop_test.rs`:

```rust
//! Interoperability test binary for Julia/Python integration.
//!
//! Usage:
//!   cargo run --example interop_test --no-default-features --features runtime-loading -- \
//!     --hdf5-lib /path/to/libhdf5.dylib \
//!     --mode read|write \
//!     --file /path/to/test.h5

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "interop_test")]
#[command(about = "HDF5 interoperability test for Julia/Python")]
struct Args {
    /// Path to HDF5 shared library
    #[arg(long)]
    hdf5_lib: PathBuf,

    /// Test mode
    #[arg(long)]
    mode: Mode,

    /// Path to HDF5 test file
    #[arg(long)]
    file: PathBuf,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    /// Read file created by Julia/Python and verify contents
    Read,
    /// Write file for Julia/Python to read
    Write,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Initialize HDF5 with the provided library path
    let lib_path = args.hdf5_lib.to_string_lossy();
    if let Err(e) = hdf5::sys::init(Some(&lib_path)) {
        eprintln!("Failed to initialize HDF5: {}", e);
        return ExitCode::FAILURE;
    }

    println!("HDF5 library loaded from: {}", lib_path);

    let result = match args.mode {
        Mode::Read => read_test_file(&args.file),
        Mode::Write => write_test_file(&args.file),
    };

    match result {
        Ok(()) => {
            println!("SUCCESS");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn read_test_file(path: &PathBuf) -> hdf5::Result<()> {
    use hdf5::File;

    let file = File::open(path)?;

    // Read scalar attribute from root group
    let root = file.group("/")?;
    let attr_value: String = root.attr("test_attr")?.read_scalar()?;
    assert_eq!(attr_value, "hello from julia/python", "Attribute mismatch");
    println!("  Attribute 'test_attr': {}", attr_value);

    // Read 1D integer dataset
    let ds_int = file.dataset("integers")?;
    let int_data: Vec<i64> = ds_int.read_raw()?;
    assert_eq!(int_data, vec![1i64, 2, 3, 4, 5], "Integer dataset mismatch");
    println!("  Dataset 'integers': {:?}", int_data);

    // Read 2D float dataset
    let ds_float = file.dataset("matrix")?;
    let float_data: ndarray::Array2<f64> = ds_float.read()?;
    let expected = ndarray::array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
    assert_eq!(float_data, expected, "Float matrix mismatch");
    println!("  Dataset 'matrix': {:?}", float_data);

    // Read string dataset
    let ds_str = file.dataset("strings")?;
    let str_data: Vec<String> = ds_str.read_raw()?;
    assert_eq!(str_data, vec!["foo", "bar", "baz"], "String dataset mismatch");
    println!("  Dataset 'strings': {:?}", str_data);

    Ok(())
}

fn write_test_file(path: &PathBuf) -> hdf5::Result<()> {
    use hdf5::File;

    let file = File::create(path)?;

    // Write scalar attribute to root group
    let root = file.group("/")?;
    let attr = root.new_attr::<hdf5::types::VarLenUnicode>().create("test_attr")?;
    attr.write_scalar(&hdf5::types::VarLenUnicode::from("hello from rust"))?;

    // Write 1D integer dataset
    let int_data = vec![10i64, 20, 30, 40, 50];
    file.new_dataset::<i64>().shape([5]).create("integers")?.write(&int_data)?;

    // Write 2D float dataset
    let float_data = ndarray::array![[1.5, 2.5, 3.5], [4.5, 5.5, 6.5]];
    file.new_dataset::<f64>().shape([2, 3]).create("matrix")?.write(&float_data)?;

    // Write string dataset
    let str_data: Vec<hdf5::types::VarLenUnicode> = vec!["rust", "test", "data"]
        .into_iter()
        .map(hdf5::types::VarLenUnicode::from)
        .collect();
    file.new_dataset::<hdf5::types::VarLenUnicode>()
        .shape([3])
        .create("strings")?
        .write(&str_data)?;

    file.flush()?;
    Ok(())
}
```

**Step 3: Build and verify the binary compiles**

Run:
```bash
cd /Users/hiroshi/projects/tensor4all/tensor4all-hdf5-ffi
cargo build --example interop_test --no-default-features --features runtime-loading
```

Expected: Build succeeds

**Step 4: Commit**

```bash
git add hdf5/Cargo.toml hdf5/examples/interop_test.rs
git commit -m "feat: add interop test binary for Julia/Python integration"
```

---

## Task 2: Create Julia Test Script

**Files:**
- Create: `tests/julia/Project.toml`
- Create: `tests/julia/test_interop.jl`

**Step 1: Create Julia project file**

Create `tests/julia/Project.toml`:

```toml
[deps]
HDF5 = "f67ccb44-e63f-5c2f-98bd-6dc0ccc4ba2f"
Test = "8dfed614-e22c-5e08-85e1-65c5234f0b40"
```

**Step 2: Create Julia test script**

Create `tests/julia/test_interop.jl`:

```julia
#!/usr/bin/env julia
using Test
using HDF5
import HDF5_jll

# Get the HDF5 library path from HDF5_jll
function get_hdf5_lib_path()
    if Sys.isapple()
        return joinpath(dirname(HDF5_jll.libhdf5_path), "libhdf5.dylib")
    elseif Sys.islinux()
        return HDF5_jll.libhdf5_path
    elseif Sys.iswindows()
        return joinpath(dirname(HDF5_jll.libhdf5_path), "hdf5.dll")
    else
        error("Unsupported platform")
    end
end

# Path to the Rust binary
function get_rust_binary()
    project_root = dirname(dirname(@__DIR__))
    if Sys.iswindows()
        return joinpath(project_root, "target", "debug", "examples", "interop_test.exe")
    else
        return joinpath(project_root, "target", "debug", "examples", "interop_test")
    end
end

# Build the Rust binary
function build_rust_binary()
    project_root = dirname(dirname(@__DIR__))
    cmd = Cmd(`cargo build --example interop_test --no-default-features --features runtime-loading`; dir=project_root)
    run(cmd)
end

@testset "HDF5 Julia-Rust Interoperability" begin
    hdf5_lib = get_hdf5_lib_path()
    @info "HDF5 library path" hdf5_lib

    # Ensure the Rust binary is built
    build_rust_binary()
    rust_binary = get_rust_binary()
    @test isfile(rust_binary) "Rust binary not found at $rust_binary"

    @testset "Julia writes, Rust reads" begin
        mktempdir() do tmpdir
            test_file = joinpath(tmpdir, "julia_to_rust.h5")

            # Julia writes the file
            h5open(test_file, "w") do f
                # Write attribute to root group
                attrs(f)["test_attr"] = "hello from julia/python"

                # Write 1D integer dataset
                f["integers"] = Int64[1, 2, 3, 4, 5]

                # Write 2D float dataset (Julia is column-major, HDF5 stores row-major)
                f["matrix"] = Float64[1.0 2.0 3.0; 4.0 5.0 6.0]

                # Write string dataset
                f["strings"] = ["foo", "bar", "baz"]
            end

            @test isfile(test_file)

            # Rust reads the file
            cmd = `$rust_binary --hdf5-lib $hdf5_lib --mode read --file $test_file`
            result = run(cmd)
            @test result.exitcode == 0
        end
    end

    @testset "Rust writes, Julia reads" begin
        mktempdir() do tmpdir
            test_file = joinpath(tmpdir, "rust_to_julia.h5")

            # Rust writes the file
            cmd = `$rust_binary --hdf5-lib $hdf5_lib --mode write --file $test_file`
            result = run(cmd)
            @test result.exitcode == 0
            @test isfile(test_file)

            # Julia reads and verifies
            h5open(test_file, "r") do f
                # Read attribute
                @test read(attrs(f)["test_attr"]) == "hello from rust"

                # Read integer dataset
                @test read(f["integers"]) == Int64[10, 20, 30, 40, 50]

                # Read float matrix
                @test read(f["matrix"]) ≈ Float64[1.5 2.5 3.5; 4.5 5.5 6.5]

                # Read string dataset
                @test read(f["strings"]) == ["rust", "test", "data"]
            end
        end
    end
end

println("\n✓ All Julia interop tests passed!")
```

**Step 3: Run Julia tests locally**

Run:
```bash
cd /Users/hiroshi/projects/tensor4all/tensor4all-hdf5-ffi/tests/julia
julia --project=. -e 'using Pkg; Pkg.instantiate()'
julia --project=. test_interop.jl
```

Expected: All tests pass

**Step 4: Commit**

```bash
git add tests/julia/
git commit -m "test: add Julia interoperability tests"
```

---

## Task 3: Create Python Test Script

**Files:**
- Create: `tests/python/requirements.txt`
- Create: `tests/python/test_interop.py`

**Step 1: Create Python requirements**

Create `tests/python/requirements.txt`:

```
h5py>=3.0
numpy>=1.20
```

**Step 2: Create Python test script**

Create `tests/python/test_interop.py`:

```python
#!/usr/bin/env python3
"""HDF5 Python-Rust interoperability tests."""

import os
import subprocess
import sys
import tempfile
from pathlib import Path

import h5py
import numpy as np


def get_hdf5_lib_path() -> str:
    """Get the path to the HDF5 shared library used by h5py."""
    # h5py stores the library path in its configuration
    # Try to find it via the HDF5 library info
    info = h5py.version.hdf5_version_tuple

    # Platform-specific library finding
    if sys.platform == "darwin":
        # macOS: look for dylib
        import ctypes.util

        # First try h5py's bundled library
        h5py_dir = Path(h5py.__file__).parent
        candidates = [
            h5py_dir / ".dylibs" / "libhdf5.dylib",
            h5py_dir / "libhdf5.dylib",
        ]

        for candidate in candidates:
            if candidate.exists():
                return str(candidate)

        # Fall back to system library
        lib = ctypes.util.find_library("hdf5")
        if lib:
            return lib

        # Homebrew paths
        homebrew_paths = [
            "/opt/homebrew/lib/libhdf5.dylib",
            "/usr/local/lib/libhdf5.dylib",
        ]
        for path in homebrew_paths:
            if os.path.exists(path):
                return path

    elif sys.platform == "linux":
        # Linux: look for .so
        import ctypes.util

        # First try h5py's bundled library
        h5py_dir = Path(h5py.__file__).parent
        candidates = list(h5py_dir.glob("*.libs/libhdf5*.so*"))
        if candidates:
            return str(candidates[0])

        # Fall back to system library
        lib = ctypes.util.find_library("hdf5")
        if lib:
            return lib

        # Common system paths
        system_paths = [
            "/usr/lib/x86_64-linux-gnu/libhdf5.so",
            "/usr/lib/libhdf5.so",
        ]
        for path in system_paths:
            if os.path.exists(path):
                return path

    elif sys.platform == "win32":
        h5py_dir = Path(h5py.__file__).parent
        candidates = list(h5py_dir.glob("*.dll")) + list(h5py_dir.glob("hdf5.dll"))
        if candidates:
            return str(candidates[0])

    raise RuntimeError("Could not find HDF5 library path")


def get_rust_binary() -> Path:
    """Get path to the Rust interop test binary."""
    project_root = Path(__file__).parent.parent.parent
    if sys.platform == "win32":
        return project_root / "target" / "debug" / "examples" / "interop_test.exe"
    else:
        return project_root / "target" / "debug" / "examples" / "interop_test"


def build_rust_binary():
    """Build the Rust binary."""
    project_root = Path(__file__).parent.parent.parent
    subprocess.run(
        [
            "cargo",
            "build",
            "--example",
            "interop_test",
            "--no-default-features",
            "--features",
            "runtime-loading",
        ],
        cwd=project_root,
        check=True,
    )


def test_python_writes_rust_reads():
    """Test that Rust can read files created by Python/h5py."""
    hdf5_lib = get_hdf5_lib_path()
    rust_binary = get_rust_binary()

    print(f"HDF5 library: {hdf5_lib}")
    print(f"Rust binary: {rust_binary}")

    with tempfile.TemporaryDirectory() as tmpdir:
        test_file = Path(tmpdir) / "python_to_rust.h5"

        # Python writes the file
        with h5py.File(test_file, "w") as f:
            # Write attribute to root group
            f.attrs["test_attr"] = "hello from julia/python"

            # Write 1D integer dataset
            f.create_dataset("integers", data=np.array([1, 2, 3, 4, 5], dtype=np.int64))

            # Write 2D float dataset
            f.create_dataset(
                "matrix", data=np.array([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], dtype=np.float64)
            )

            # Write string dataset (variable-length UTF-8)
            dt = h5py.string_dtype(encoding="utf-8")
            f.create_dataset("strings", data=["foo", "bar", "baz"], dtype=dt)

        assert test_file.exists()

        # Rust reads the file
        result = subprocess.run(
            [
                str(rust_binary),
                "--hdf5-lib",
                hdf5_lib,
                "--mode",
                "read",
                "--file",
                str(test_file),
            ],
            capture_output=True,
            text=True,
        )

        print(result.stdout)
        if result.returncode != 0:
            print(result.stderr, file=sys.stderr)
        assert result.returncode == 0, f"Rust read failed: {result.stderr}"

    print("✓ Python writes, Rust reads: PASSED")


def test_rust_writes_python_reads():
    """Test that Python can read files created by Rust."""
    hdf5_lib = get_hdf5_lib_path()
    rust_binary = get_rust_binary()

    with tempfile.TemporaryDirectory() as tmpdir:
        test_file = Path(tmpdir) / "rust_to_python.h5"

        # Rust writes the file
        result = subprocess.run(
            [
                str(rust_binary),
                "--hdf5-lib",
                hdf5_lib,
                "--mode",
                "write",
                "--file",
                str(test_file),
            ],
            capture_output=True,
            text=True,
        )

        print(result.stdout)
        if result.returncode != 0:
            print(result.stderr, file=sys.stderr)
        assert result.returncode == 0, f"Rust write failed: {result.stderr}"
        assert test_file.exists()

        # Python reads and verifies
        with h5py.File(test_file, "r") as f:
            # Read attribute
            attr_value = f.attrs["test_attr"]
            if isinstance(attr_value, bytes):
                attr_value = attr_value.decode("utf-8")
            assert attr_value == "hello from rust", f"Attribute mismatch: {attr_value}"

            # Read integer dataset
            integers = f["integers"][:]
            np.testing.assert_array_equal(integers, [10, 20, 30, 40, 50])

            # Read float matrix
            matrix = f["matrix"][:]
            expected = np.array([[1.5, 2.5, 3.5], [4.5, 5.5, 6.5]])
            np.testing.assert_array_almost_equal(matrix, expected)

            # Read string dataset
            strings = [s.decode("utf-8") if isinstance(s, bytes) else s for s in f["strings"][:]]
            assert strings == ["rust", "test", "data"], f"String mismatch: {strings}"

    print("✓ Rust writes, Python reads: PASSED")


def main():
    print("=" * 60)
    print("HDF5 Python-Rust Interoperability Tests")
    print("=" * 60)

    # Build the Rust binary first
    print("\nBuilding Rust binary...")
    build_rust_binary()

    rust_binary = get_rust_binary()
    assert rust_binary.exists(), f"Rust binary not found at {rust_binary}"

    print("\nRunning tests...")
    test_python_writes_rust_reads()
    test_rust_writes_python_reads()

    print("\n" + "=" * 60)
    print("✓ All Python interop tests passed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
```

**Step 3: Run Python tests locally**

Run:
```bash
cd /Users/hiroshi/projects/tensor4all/tensor4all-hdf5-ffi/tests/python
pip install -r requirements.txt  # or use existing venv
python test_interop.py
```

Expected: All tests pass

**Step 4: Commit**

```bash
git add tests/python/
git commit -m "test: add Python interoperability tests"
```

---

## Task 4: Update CI Workflow

**Files:**
- Modify: `.github/workflows/ci.yml`

**Step 1: Add Julia and Python interop test jobs**

Add to `.github/workflows/ci.yml`:

```yaml
  interop-julia:
    name: Julia interop
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install Julia
        uses: julia-actions/setup-julia@v2
        with:
          version: '1.10'
      - name: Install HDF5 (system, for building)
        run: sudo apt-get update && sudo apt-get install -y libhdf5-dev
      - name: Setup Julia project
        run: |
          cd tests/julia
          julia --project=. -e 'using Pkg; Pkg.instantiate()'
      - name: Run Julia interop tests
        run: |
          cd tests/julia
          julia --project=. test_interop.jl

  interop-python:
    name: Python interop
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Install HDF5 (system, for building)
        run: sudo apt-get update && sudo apt-get install -y libhdf5-dev
      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.11'
      - name: Install Python dependencies
        run: |
          pip install -r tests/python/requirements.txt
      - name: Run Python interop tests
        run: |
          python tests/python/test_interop.py
```

**Step 2: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add Julia and Python interoperability tests"
```

---

## Task 5: Local Verification and Final Commit

**Step 1: Run all tests locally**

```bash
# Rust tests
cargo test --workspace

# Julia tests
cd tests/julia
julia --project=. -e 'using Pkg; Pkg.instantiate()'
julia --project=. test_interop.jl

# Python tests
cd ../python
pip install -r requirements.txt
python test_interop.py
```

**Step 2: Format and lint**

```bash
cargo fmt --all
cargo clippy --workspace
```

**Step 3: Push and verify CI**

```bash
git push
# Monitor CI: gh pr checks or gh run list
```
