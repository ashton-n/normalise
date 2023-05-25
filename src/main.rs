use std::env;
use std::ffi::OsStr;
use std::fmt::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
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
    println!("test path: {:?}\n", sample_pair_paths[3]);
    let test = data_ops::normalise_sample(&sample_pair_paths[3]).unwrap();

    /*let data = sample_pair_paths.iter()
                                .enumerate()
                                .map(|(idx, path_pair)| {
                                    // CAREFUL: idx needs to be sequential using MPI
                                    (idx, data_ops::normalise_sample(path_pair).unwrap()) 
                                })
                                .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>();*/


    //println!("{:?}",data);

    let data_test: HashMap<usize, HashMap<usize,(u8, u8)>> = vec![(0_usize, test)].into_iter().collect();
    let status = write_to_files::write_to_bin(&output_filename, data_test).unwrap();
    println!("{:?}", status);
    
    //data_test.iter().for_each(|(indiv_idx, indiv_val)| {
    //    println!("{:#?}", indiv_val.get(&0));
    //});

    /*let nested_map = HashMap::from([
        ("key1", HashMap::from([
            ("nested_key1", "value1"),
            ("nested_key2", "value2"),
        ])),
        ("key2", HashMap::from([
            ("nested_key3", "value3"),
            ("nested_key4", "value4"),
        ])),
    ]);*/

    //let transposed_data = data_ops::transpose(data_test).unwrap();
    //println!("{:?}", transposed_data);
    //transposed_data

    //println!("{:?}", o);
    
}
