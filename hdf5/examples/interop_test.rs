//! Interoperability test binary for Julia/Python integration.
//!
//! Usage:
//!   cargo run --example interop_test --features runtime-loading -- \
//!     --hdf5-lib /path/to/libhdf5.dylib \
//!     --mode read|write \
//!     --file /path/to/test.h5
//!
//! Note: The runtime-loading feature allows specifying the HDF5 library path at runtime,
//! but currently requires the default `link` feature for compilation.

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
    use hdf5::types::VarLenUnicode;
    use hdf5::File;
    use std::str::FromStr;

    let file = File::open(path)?;

    // Read scalar attribute from root group
    let root = file.group("/")?;
    let attr_value: VarLenUnicode = root.attr("test_attr")?.read_scalar()?;
    let expected_attr = VarLenUnicode::from_str("hello from julia/python").unwrap();
    assert_eq!(attr_value, expected_attr, "Attribute mismatch");
    println!("  Attribute 'test_attr': {}", attr_value.as_str());

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
    let str_data: Vec<VarLenUnicode> = ds_str.read_raw()?;
    let expected_strs: Vec<VarLenUnicode> = vec!["foo", "bar", "baz"]
        .into_iter()
        .map(|s| VarLenUnicode::from_str(s).unwrap())
        .collect();
    assert_eq!(str_data, expected_strs, "String dataset mismatch");
    println!("  Dataset 'strings': {:?}", str_data.iter().map(|s| s.as_str()).collect::<Vec<_>>());

    Ok(())
}

fn write_test_file(path: &PathBuf) -> hdf5::Result<()> {
    use hdf5::types::VarLenUnicode;
    use hdf5::File;
    use std::str::FromStr;

    let file = File::create(path)?;

    // Write scalar attribute to root group
    let root = file.group("/")?;
    let attr = root.new_attr::<VarLenUnicode>().create("test_attr")?;
    let attr_value = VarLenUnicode::from_str("hello from rust").unwrap();
    attr.write_scalar(&attr_value)?;

    // Write 1D integer dataset
    let int_data = vec![10i64, 20, 30, 40, 50];
    file.new_dataset::<i64>().shape([5]).create("integers")?.write(&int_data)?;

    // Write 2D float dataset
    let float_data = ndarray::array![[1.5, 2.5, 3.5], [4.5, 5.5, 6.5]];
    file.new_dataset::<f64>().shape([2, 3]).create("matrix")?.write(&float_data)?;

    // Write string dataset
    let str_data: Vec<VarLenUnicode> = vec!["rust", "test", "data"]
        .into_iter()
        .map(|s| VarLenUnicode::from_str(s).unwrap())
        .collect();
    file.new_dataset::<VarLenUnicode>().shape([3]).create("strings")?.write(&str_data)?;

    file.flush()?;
    Ok(())
}
