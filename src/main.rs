use std::env;
use std::fmt::Error;

use std::path::{PathBuf};
//use std::time::Instant;
//mod bit_ops;
mod data_ops;
mod read_from_files;
mod write_to_files;
//mod view_data;
use std::collections::HashMap;

// reads in parameters passed to programme
fn read_term_args() -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Invalid input: input must be: cargo run -- [input file directory] [output filename]");
        return Err(Box::new(Error));
    } else {
        let input_dir = std::env::args().nth(1).unwrap();
        let output_filename = std::env::args().nth(2).unwrap();
        let input_dir = PathBuf::from(&input_dir);
        //let output_filename = PathBuf::from(format!("./{}{}", &output_filename, ".dat"));
        let mut output_filename = PathBuf::from(output_filename);
        output_filename.set_extension("dat");

        Ok((input_dir, output_filename))
    } 
}

fn main() {
    // fetch user inputs
    let (input_dir, output_filename) = read_term_args().unwrap();
    println!("{:?} {:?}", input_dir, output_filename);

    // fetch files pathes and sort into files pairs per sample
    let sample_pair_paths = read_from_files::get_file_pathes(&input_dir).unwrap();
    if sample_pair_paths.is_empty() {println!("Input directory is empty or no files with '.dat' extension found")}
    
    //println!("{:#?}", sample_pair_paths);
    //println!("{:#?}", sample_pair_paths.len());
    // calculate per 
    //println!("test path: {:?}\n", sample_pair_paths[3]);
    //let test = data_ops::normalise_sample(&sample_pair_paths[3]).unwrap();

    //println!("test path: {:?}\n", sample_pair_paths[3]);
    let data = sample_pair_paths.into_iter()
                                .enumerate()
                                .map(|(idx, green_red_paths)| {
                                    (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
                                })
                                .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 

    let status = write_to_files::write_to_bin(&output_filename, data).unwrap();
    println!("{:?}", status);
    
    
}
