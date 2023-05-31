use std::env;
use std::fmt::Error;
use std::path::PathBuf;
use std::time::Instant;
mod data_ops;
mod read_from_files;
mod write_to_files;
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
        let mut output_filename = PathBuf::from(output_filename);
        output_filename.set_extension("dat");

        Ok((input_dir, output_filename))
    } 
}

fn main() {
    // read in parameters passed to the programme
    let (input_dir, output_filename) = read_term_args().expect("WHAT!");
    
    // get the paths of the files to be read
    let mut sample_pair_paths = read_from_files::get_file_pathes(&input_dir).unwrap();
    
    // if no files with '.dat' extension are found, exit the programme
    if sample_pair_paths.is_empty() {println!("Input directory is empty or no files with '.dat' extension found")}

    // initialise MPI
    let universe = mpi::initialize().unwrap();
    
    // get the world communicator
    let world = universe.world();
    
    // define the root process
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);
    
    // get the number of processes 
    let count = world.size() as usize;
    
    // get the rank of the current process
    let rank = world.rank();

    // check that the number of processes is less than or equal to the number of samples
    if count > sample_pair_paths.len() {
        panic!("Number of processes must be less than or equal to the number of samples");
    }

    // split the pathes of the files to be read into chunks
    // so that each process can read in its own chunk of data.
    // and store the remainder data in a separate variable
    // so that it can be processed by the root process later
    let (chunks, remainder) = if sample_pair_paths.len() % count != 0 {
        let remainder = sample_pair_paths.remove(sample_pair_paths.len()-1);
        let chunk_size = sample_pair_paths.len() / count;
        let chunks = sample_pair_paths.chunks(chunk_size)
                                      .map(|chunk| chunk.to_vec())
                                      .collect::<Vec<_>>();
        (chunks, Some(remainder))
    } else {
        let chunk_size = sample_pair_paths.len() / count;
        let chunks = sample_pair_paths.chunks(chunk_size)
                                      .map(|chunk| chunk.to_vec())        
                                      .collect::<Vec<_>>();
        (chunks, None)
    };
    let start = Instant::now();
    // let each rank process its chunk of data
    if world.rank() == root_rank {
        
        // read in the data from the files and normalise it so that each SNP is represented by a u8
        // and each sample is represented by a vector of u8's and store it in a nested hashmap
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        
        // get the number of individuals
        let snp_total = data[0].len();
        
        // convert data into a vector of u8's
        //let data_as_vec = data_ops::hash_to_vec(data);
        let data_as_vec = data.into_iter().flatten().flatten().collect::<Vec<u8>>();

        // define an appropriately sized vector to store the gathered data
        let mut processed_data = vec![0u8; data_as_vec.len()*count];
        
        // gather data from all processes into the root process
        root_process.gather_into_root(&data_as_vec[..], &mut processed_data[..]);

        // the root process will have
        // remainder data. That remainder data is captured here   
        let processed_data = if remainder.is_some() {
            let remainder = vec![vec![remainder.unwrap()]];
            let remainder = data_ops::normalise_data_from_file_chunks(remainder, rank);
            let mut remainder = remainder.into_par_iter().flatten().flatten().collect::<Vec<u8>>();
            
          
            processed_data.append(&mut remainder);
            processed_data
        } else {
            processed_data
        };

        // transpose the data so that each row is a sample and each column is a SNP
        let processed_data = data_ops::vec_transpose(processed_data, snp_total);
        
        // write the processed data to a binary file
        write_to_files::write_vec_to_bin(&output_filename, processed_data).unwrap();
        println!("Completed in: {:?}", start.elapsed());
    } else {
        // let each rank process its chunk
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        
        // convert data from a nested hashmap to a vector of u8's
        let data_as_vec = data.into_iter().flatten().flatten().collect::<Vec<u8>>();
        
        // gather data from all processes into the root process
        root_process.gather_into(&data_as_vec[..]);
    }
}
