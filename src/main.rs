use std::time::Instant;
use rayon::prelude::*;
use mpi::traits::*;
mod data_ops;
mod read_from_files;
mod write_to_files;
mod term_data;

fn main() {
    // read in input directory and output filename from terminal
    let (input_dir, output_filename) = term_data::read_term_args()
        .expect("Invalid input: input must be: cargo run -- [input file directory] [output filename]");
    
    // get a vector of the pathes of the red and green sample files
    let sample_pair_paths = read_from_files::get_file_pathes(&input_dir)
        .expect("Error reading input directory");
    
    // if no files with '.dat' extension are found, exit the programme
    if sample_pair_paths.is_empty() {
        println!("Input directory is empty or no files with '.dat' extension found")
    }

    // initialise MPI
    let universe = mpi::initialize().unwrap();
    
    // get the world communicator
    let world = universe.world();
    
    // define the root process
    let root_rank = 0;
    let root_process = world.process_at_rank(root_rank);
    
    // get the number of processes 
    let num_procs = world.size() as usize;
    
    // get the rank of the current process
    let rank = world.rank();

    // check that the number of processes is less than or equal to the number of samples
    if num_procs > sample_pair_paths.len() {
        panic!("Number of processes must be less than or equal to the number of samples");
    }

    // split the pathes of the files to be read, into chunks
    // so that each process can read in its own chunk of data.
    let (chunks, remainder) = read_from_files::portion_input_data(sample_pair_paths, num_procs);
    
    // begin timing the programme
    let start = Instant::now();

    // let each rank, process its own chunk of data
    if world.rank() == root_rank {
        
        // read in and normalise the data from the file paths allocated
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        
        // get the number of snps per sample
        let snp_total = data[0].len();
        
        // flatten the nested vector of data into a single vector
        let data_as_vec = data.into_iter().flatten().flatten().collect::<Vec<u8>>();

        // define an appropriately sized vector to store the gathered data
        let mut processed_data = vec![0u8; data_as_vec.len()*num_procs];
        
        // gather data from all processes into the root process
        root_process.gather_into_root(&data_as_vec[..], &mut processed_data[..]);

        // process the remainder of the non divisible by num_procs data  
        let processed_data = if remainder.is_some() {
            // convert the remainder into a pattern that can be processed 
            // by the normalise_data_from_file_chunks function
            let remainder = vec![vec![remainder.unwrap()]];
            let remainder = data_ops::normalise_data_from_file_chunks(remainder, rank);
            
            // flatten the nested vector of data into a single vector
            let mut remainder = remainder.into_par_iter().flatten().flatten().collect::<Vec<u8>>();
            
            // append the remainder to the processed data
            processed_data.append(&mut remainder);

            // return the appended processed data
            processed_data
        } else {
            // return the non-appended processed data
            processed_data
        };

        // transpose the data so that each row is a sample and each column is a SNP
        let processed_data = data_ops::vec_transpose(processed_data, snp_total);
        
        // write the processed data to a binary file
        write_to_files::write_vec_to_bin(&output_filename, processed_data).expect("Error writing to file");
        
        // print the time taken to process the data
        println!("Completed in: {:?}", start.elapsed());
    } else {
        // read in and normalise the data from the file paths allocated
        let data = data_ops::normalise_data_from_file_chunks(chunks, rank);
        
        // flatten the nested vector of data into a single vector
        let data_as_vec = data.into_iter().flatten().flatten().collect::<Vec<u8>>();
        
        // send the data from all processes to the root process
        root_process.gather_into(&data_as_vec[..]);
    }
}
