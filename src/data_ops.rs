use std::path::{PathBuf};
use std::error;

use rayon::prelude::*;

// Return the n minimum and maximum values of a vector
// where the values can be integers or floats.
pub fn get_min_max_v<T: PartialOrd + Copy + Sync, V: AsRef<[T]>>(value_map: V, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    
    // convert the value_map to a reference  
    let value_map = value_map.as_ref();

    // create a vector of indices
    let mut indices: Vec<usize> = (0..value_map.len()).collect();
    
    // sort the indices from smallest to largest in parallel
    indices.par_sort_by(|&a, &b| {
        let cmp = value_map[a].partial_cmp(&value_map[b]).unwrap_or(std::cmp::Ordering::Equal);
        
        // if the values are equal, sort by index but in reverse order
        if cmp == std::cmp::Ordering::Equal {
            b.cmp(&a)
        } else {
            cmp
        }
    });
    
    // get the first n and last n indices
    let min_indices = indices[0..n].to_vec();
    let max_indices = indices[(indices.len() - n)..].to_vec();

    // return the indices
    Ok(min_indices.into_iter().chain(max_indices.into_iter()).collect())    
}

// normalises the data by removing the smallest and largest 5 values from red, green and ratio (red/green)
// and then subtracting the smallest value from all values (translating the data)
// and then dividing all values by the largest value (scaling the data)
// finally, the data is converted to a vector of vectors
// where each vector is a sample and each element of the vector is a value for a snp 
pub fn normalise_sample(path: &(PathBuf, PathBuf)) -> Result<Vec<Vec<u8>>, Box<dyn error::Error>>  {
    
    // read the red and green files into vectors
    let mut red = std::fs::read(&path.1).expect("Error reading red for indiv...");
    let mut green = std::fs::read(&path.0).expect("Error reading green for indiv...");

    // check that the two vectors are of the same length
    assert_eq!(green.len(), red.len());

    // create the ratio vector of red / green 
    let ratio = green.iter()
                     .zip(red.iter())
                     .map(|(g, r)| *r as f32 / *g as f32)
                     .collect::<Vec<f32>>();

    // get the indices of the smallest and largest 5 values in each vector
    let green_min_max = get_min_max_v(&green, 5).expect("Error getting min max for green");
    let red_min_max = get_min_max_v(&red, 5).expect("Error getting min max for red");
    let ratio_min_max = get_min_max_v(&ratio, 5).expect("Error getting min max for ratio");

    // put all indices to be removed into a vector
    let mut indices_to_rmv = green_min_max.iter()
                                          .chain(red_min_max.iter())
                                          .chain(ratio_min_max.iter())
                                          .collect::<Vec<&usize>>();

    // check to verify that there are 30 values in total 
    assert_eq!(indices_to_rmv.len(), 30);

    // sort the indices to make removal easier
    indices_to_rmv.sort();

    // remove duplicates
    indices_to_rmv.dedup();

    // reverse the vector so that the removal of the indices does 
    // not affect the indices of the other values
    indices_to_rmv.reverse();

    // remove the indices from the vectors
    indices_to_rmv.iter().for_each(|x| {
        green.remove(**x);
        red.remove(**x);
    });

    // check that the two vectors are still of the same length
    assert_eq!(green.len(), red.len());

    // get the smallest values in each vector
    let r0 = *red.iter().min().expect("Error getting min red value");
    let g0 = *green.iter().min().expect("Error getting min green value");

    // translate the data by subtracting the smallest value from all values
    let mut green = green.into_iter()
                         .map(|val| val.saturating_sub(g0)) 
                         .collect::<Vec<u8>>(); //underflow here
    
    let mut red = red.into_iter()
                     .map(|val| val.saturating_sub(r0)) 
                     .collect::<Vec<u8>>(); //underflow here
    
    // reverse the vectors so that the largest value is at the front
    // to make insertion easier
    indices_to_rmv.reverse();

    // insert the removed indices back setting the values to 255
    indices_to_rmv.iter().for_each(|x| {
        green.insert(**x, 255_u8);
        red.insert(**x, 255_u8);
    });
    
    // zip the vectors together to create a vector of vectors
    let data = green.into_iter()
                    .zip(red.into_iter())
                    .map(|(green, red)| vec![green, red])
                    .collect::<Vec<Vec<u8>>>();
    
    // return the normalised data
    Ok(data)

}

// transpose the data so that each row is a sample and each column is a SNP
pub fn vec_transpose(data: Vec<u8>, no_snps: usize) -> Vec<u8> {
    
    // group the data into grean and red pairs of 2
    let data = data.chunks_exact(2).map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();
    
    // group the pairs into samples of no_snps
    let data = data.chunks_exact(no_snps).map(|x| x.to_vec()).collect::<Vec<Vec<Vec<u8>>>>();

    // transpose the data
    let trans_vec = (0..no_snps).into_iter()
        .flat_map(|snp_idx| {
            data.iter().map(move |all_indiv_snps| {
                all_indiv_snps[snp_idx].clone()
            })
        })
        .flatten()
        .collect::<Vec<u8>>();

    // return the transposed data
    trans_vec
}

// read in and normalise the data from the file paths allocated
pub fn normalise_data_from_file_chunks(chunks: Vec<Vec<(PathBuf, PathBuf)>>, rank: i32) -> Vec<Vec<Vec<u8>>> {
    chunks[rank as usize].iter()
                         .map(|green_red_paths| {
                             // normalise each allocated sample of data
                             normalise_sample(&green_red_paths).expect("Error normalising sample...")
                         })
                         .collect::<Vec<Vec<Vec<u8>>>>()     
}
