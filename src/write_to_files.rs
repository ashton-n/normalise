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
