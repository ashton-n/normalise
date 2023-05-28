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


use mpi::datatype::{MutView, UserDatatype, View};
use mpi::traits::*;
use mpi::Count;

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
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    // fetch user inputs
    let (input_dir, output_filename) = read_term_args().unwrap();
    //println!("{:?} {:?}", input_dir, output_filename);

    // fetch files pathes and sort into files pairs per sample
    let sample_pair_paths = read_from_files::get_file_pathes(&input_dir).unwrap();
    if sample_pair_paths.is_empty() {println!("Input directory is empty or no files with '.dat' extension found")}
    
    //println!("{:#?}", sample_pair_paths);
    //println!("{:#?}", sample_pair_paths.len());
    // calculate per 
    //println!("test path: {:?}\n", sample_pair_paths[3]);
    //let test = data_ops::normalise_sample(&sample_pair_paths[3]).unwrap();

    // -----------------------
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);
    let count = world.size() as usize;
    let rank = world.rank();


    // divide sample_pair_paths into chunks of equal sizes for each process
    let chunk_size = sample_pair_paths.len() / count;
    let chunks = sample_pair_paths.chunks(chunk_size).collect::<Vec<_>>();
    
    if world.rank() == root_rank {
        println!("Rank: {:?} {:#?}", rank, chunks[rank as usize]);
        // let each rank process its chunk
        let data = chunks[rank as usize].into_iter()
                                        .enumerate()
                                        .map(|(idx, green_red_paths)| {
                                            (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
                                        })
                                        .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 
        // convert data into a vector of u8's
        let decomposed_data = data_ops::decompose(data);
        // define a vector to store the gathered data
        let mut processed_data = vec![0u8; decomposed_data.len()*count];
        root_process.gather_into_root(&decomposed_data[..], &mut processed_data[..]);
        

        processed_data.chunks(100).for_each(|x| {
            println!("{:?}", x)
        });

        //println!("\n{:?}", processed_data);
        //write_to_files::write_vec_to_bin(&output_filename, processed_data).unwrap();

    } else {
        println!("Rank: {:?} {:#?}", rank, chunks[rank as usize]);
        // let each rank process its chunk
        let data = chunks[rank as usize].into_iter()
                                        .enumerate()
                                        .map(|(idx, green_red_paths)| {
                                            (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
                                        })
                                        .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 
        // convert data into a vector of u8's
        let decomposed_data = data_ops::decompose(data);
        root_process.gather_into(&decomposed_data[..]);
    }





    //decomposed_data.iter().for_each(|vec| println!("{:?}", vec));
    //println!("{:#?}", decomposed_data);
    
    //if rank == root_rank {
        // gather into root process using gather_into_root
        
        
    //}

    //else {
        // send to root process
        
    //}

    


    




    //println!("test path: {:?}\n", sample_pair_paths[3]);
    //let data = sample_pair_paths.into_iter()
    //                            .enumerate()
    //                            .map(|(idx, green_red_paths)| {
    //                                (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
    //                            })
    //                            .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 
    //
    //let test = data_ops::decompose(data);
    //println!("{:#?}", test);

    //let status = write_to_files::write_to_bin(&output_filename, data).unwrap();
    //println!("{:?}", status);
    
    
}
