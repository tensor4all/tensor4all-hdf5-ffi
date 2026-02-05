use hdf5_rt::File;
use std::path::Path;

fn main() {
    let path = "/tmp/test.h5";

    // Remove existing file if any
    let _ = std::fs::remove_file(path);

    println!("Creating file at: {}", path);
    match File::create(path) {
        Ok(file) => {
            println!("File created successfully!");
            println!("File id: {:?}", file.id());
            drop(file);
        }
        Err(e) => {
            println!("Error creating file: {}", e);
        }
    }

    // Cleanup
    let _ = std::fs::remove_file(path);
}
