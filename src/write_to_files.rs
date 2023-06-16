use std::fs::{OpenOptions};
use std::io::{Write, BufWriter};
use std::io;
use std::path::{PathBuf};

// write a vector of u8s to a binary file
pub fn write_vec_to_bin(write_location: &PathBuf, data: Vec<u8>) -> io::Result<String> {
    // create a buffered writer
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(write_location)
            .expect("output file path incorrect")
    );

    // write the data to the file
    data.into_iter().for_each(|x| {
        file.write(&[x]).expect("Error writing to file");
    });
    
    // flush the buffer
    file.flush()?;

    // return a string to indicate that the write was successful
    Ok(String::from("Write Complete."))
}

#[test]
fn test_write_vec_to_bin() {

    // create a test vector
    let test_vec = vec![1, 2, 3, 4, 5];

    // create a test file
    let test_file = PathBuf::from("test_file.dat");

    // write the test vector to the test file
    write_vec_to_bin(&test_file, test_vec).expect("Error writing to file");

    // create a vector to store the data read from the test file
    let mut read_vec = Vec::new();

    // read the data from the test file
    let mut file = OpenOptions::new()
                               .read(true)
                               .open(&test_file)
                               .expect("Error opening file");
    io::Read::read_to_end(&mut file, &mut read_vec).expect("Error reading file");

    // remove the test file
    std::fs::remove_file(&test_file).expect("Error removing test file");

    // check that the data read from the file is the same as the data written to the file
    assert_eq!(read_vec, vec![1, 2, 3, 4, 5]);
}