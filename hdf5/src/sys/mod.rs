//! HDF5 FFI abstraction layer.
//!
//! This module provides an abstraction over HDF5 FFI that supports two modes:
//! - Link mode (default): Uses hdf5-sys (build-time linking)
//! - Runtime-loading mode: Loads HDF5 at runtime via dlopen
//!
//! For link mode, this simply re-exports from hdf5_sys.
//! For runtime-loading mode, functions are loaded dynamically.

// In link mode (default), re-export everything from hdf5_sys
#[cfg(not(feature = "runtime-loading"))]
pub use hdf5_sys::*;

// Runtime-loading mode
#[cfg(feature = "runtime-loading")]
mod runtime;

#[cfg(feature = "runtime-loading")]
pub use runtime::*;

/// Initialize HDF5 library.
///
/// In link mode, this is a no-op (library is always available).
/// In runtime-loading mode, this loads the library from the specified path.
#[cfg(not(feature = "runtime-loading"))]
pub fn init(_path: Option<&str>) -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "runtime-loading")]
pub fn init(path: Option<&str>) -> Result<(), String> {
    runtime::init(path)
}

/// Check if the HDF5 library is initialized.
#[cfg(not(feature = "runtime-loading"))]
pub fn is_initialized() -> bool {
    true
}

#[cfg(feature = "runtime-loading")]
pub fn is_initialized() -> bool {
    runtime::is_initialized()
}

/// Get the library path (only meaningful in runtime-loading mode).
#[cfg(not(feature = "runtime-loading"))]
pub fn library_path() -> Option<String> {
    None
}

#[cfg(feature = "runtime-loading")]
pub fn library_path() -> Option<String> {
    runtime::library_path()
}
