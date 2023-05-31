use std::env;
use std::fmt::Error;
use std::path::PathBuf;

// reads in parameters passed to programme
pub fn read_term_args() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    
    // get environment arguments
    let args: Vec<String> = env::args().collect();
    
    // check that the correct number of arguments have been passed
    if args.len() != 3 {
        println!("Invalid input: input must be: cargo run -- [input file directory] [output filename]");
        return Err(Box::new(Error));
    } else {
        
        // get input directory string and convert to PathBuf
        let input_dir = std::env::args().nth(1).expect("No input directory provided");
        let input_dir = PathBuf::from(&input_dir);
        
        // get output filename add extention and convert to PathBuf
        let output_filename = std::env::args().nth(2).expect("No output filename provided");
        let mut output_filename = PathBuf::from(output_filename);
        output_filename.set_extension("dat");

        // return input directory and output filename
        Ok((input_dir, output_filename))
    } 
}

