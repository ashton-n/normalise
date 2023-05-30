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
use mpi::traits::*;
use rayon::prelude::*;

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
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    // fetch user inputs
    let (input_dir, output_filename) = read_term_args().unwrap();
    //println!("{:?} {:?}", input_dir, output_filename);

    // fetch files pathes and sort into files pairs per sample
    let mut sample_pair_paths = read_from_files::get_file_pathes(&input_dir).unwrap();
    if sample_pair_paths.is_empty() {println!("Input directory is empty or no files with '.dat' extension found")}
    //sample_pair_paths.remove(sample_pair_paths.len()-1);
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
    let (chunks, remainder) = if sample_pair_paths.len() % count != 0 {
        let remainder = sample_pair_paths.remove(sample_pair_paths.len()-1); 
        //println!("Remainder: {:?}", remainder);
        let chunk_size = sample_pair_paths.len() / count;
        let chunks = sample_pair_paths.chunks(chunk_size).collect::<Vec<_>>();
        (chunks, Some(remainder))
    } else {
        let chunk_size = sample_pair_paths.len() / count;
        let chunks = sample_pair_paths.chunks(chunk_size).collect::<Vec<_>>();
        (chunks, None)
    };
    
    if world.rank() == root_rank {
        //println!("Rank: {:?} {:#?}", rank, chunks[rank as usize]);
        // let each rank process its chunk
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        //let data = chunks[rank as usize].into_iter()
        //                                .enumerate()
        //                                .map(|(idx, green_red_paths)| {
        //                                    (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
        //                                })
        //                                .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 
        let snp_total = data[&0].len();
        // convert data into a vector of u8's
        //let decomposed_data = data_ops::decompose(data);
        let data_as_vec = data_ops::hash_to_vec(data);

        //println!("Rank: {:?} yields {:?}", world.rank(), decomposed_data);
        // define a vector to store the gathered data
        let mut processed_data = vec![0u8; data_as_vec.len()*count];
        root_process.gather_into_root(&data_as_vec[..], &mut processed_data[..]);

        //remainder
        
        //let data_chunks = processed_data.into_par_iter().chunks(chunk_size).collect::<Vec<Vec<u8>>>();
        //println!("Gathered \n{:?}", processed_data);

        //println!("snp_total \n{:?} \nno_indivs \n{:?}", snp_total, no_indivs);
        let processed_data = data_ops::vec_transpose(processed_data, snp_total);

        println!("Final \n{:?}", processed_data);

        //let processed_data = if count == sample_pair_paths.len() {
        //    println!("\nsnp_total: {:#?}", snp_total);
        //    data_ops::vec_transpose(processed_data, snp_total)
        //} else {
        //    processed_data
        //};

        write_to_files::write_vec_to_bin(&output_filename, processed_data).unwrap();

    } else {
        //println!("Rank: {:?} {:#?}", rank, chunks[rank as usize]);
        // let each rank process its chunk
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        //let data = chunks[rank as usize].into_iter()
        //                                .enumerate()
        //                                .map(|(idx, green_red_paths)| {
        //                                    (idx, data_ops::normalise_sample(&green_red_paths).unwrap())
        //                                })
        //                                .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>(); 
        // convert data into a vector of u8's
        //let decomposed_data = data_ops::decompose(data);
        let data_as_vec = data_ops::hash_to_vec(data);
        //println!("Rank: {:?} yields {:?}", world.rank(), decomposed_data);
        root_process.gather_into(&data_as_vec[..]);
    }
}
