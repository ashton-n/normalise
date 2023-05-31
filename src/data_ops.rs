
use std::path::{PathBuf};
use std::error;
//use std::collections::{HashMap};

use rayon::prelude::*;

/*pub fn get_min_max<T: PartialOrd + Copy + Sync>(value_map: &HashMap<usize, T>, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut indices: Vec<usize> = value_map.keys().copied().collect();
    indices.par_sort_by(|&a, &b| {
        let cmp = value_map[&a].partial_cmp(&value_map[&b]).unwrap_or(std::cmp::Ordering::Equal);
        if cmp == std::cmp::Ordering::Equal {
            b.cmp(&a)
        } else {
            cmp
        }
    });
    
    let min_indices = indices[0..n].to_vec();
    let max_indices = indices[(indices.len() - n)..].to_vec();

    Ok(min_indices.into_iter().chain(max_indices.into_iter()).collect())
}*/

pub fn get_min_max_v<T: PartialOrd + Copy + Sync, V: AsRef<[T]>>(value_map: V, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let value_map = value_map.as_ref();
    let mut indices: Vec<usize> = (0..value_map.len()).collect();
    indices.par_sort_by(|&a, &b| {
        let cmp = value_map[a].partial_cmp(&value_map[b]).unwrap_or(std::cmp::Ordering::Equal);
        if cmp == std::cmp::Ordering::Equal {
            b.cmp(&a)
        } else {
            cmp
        }
    });
    
    let min_indices = indices[0..n].to_vec();
    let max_indices = indices[(indices.len() - n)..].to_vec();

    Ok(min_indices.into_iter().chain(max_indices.into_iter()).collect())    
}


pub fn normalise_sample(path: &(PathBuf, PathBuf)) -> Result<Vec<Vec<u8>>, Box<dyn error::Error>>  {
    // read file to vector
    let mut red = std::fs::read(&path.1).expect("Error reading red for indiv...");
    let mut green = std::fs::read(&path.0).expect("Error reading green for indiv...");


    assert_eq!(green.len(), red.len());

    let ratio = green.iter()
                     .zip(red.iter())
                     .map(|(g, r)| *r as f32 / *g as f32)
                     .collect::<Vec<f32>>();

    let green_min_max = get_min_max_v(&green, 5).unwrap();
    let red_min_max = get_min_max_v(&red, 5).unwrap();
    let ratio_min_max = get_min_max_v(&ratio, 5).unwrap(); 

    // put all indices to be removed into a vector
    let mut indices_to_rmv = green_min_max.iter()
                                          .chain(red_min_max.iter())
                                          .chain(ratio_min_max.iter())
                                          .collect::<Vec<&usize>>();

    // small check to verify that there are 30 values in total 
    assert_eq!(indices_to_rmv.len(), 30);

    indices_to_rmv.sort();

    indices_to_rmv.dedup();

    indices_to_rmv.reverse();

    indices_to_rmv.iter().for_each(|x| {
        green.remove(**x);
        red.remove(**x);
    });

    assert_eq!(green.len(), red.len());

    // indices not required to know the min max values
    let r0 = *red.iter().min().unwrap();
    let g0 = *green.iter().min().unwrap();


    let mut green = green.into_iter()
                         .map(|val| val.saturating_sub(g0)) 
                         .collect::<Vec<u8>>(); //underflow here
    
    let mut red = red.into_iter()
                     .map(|val| val.saturating_sub(r0)) 
                     .collect::<Vec<u8>>(); //underflow here
    
    //let mut red = red.into_par_iter().enumerate().map(|(idx, mut val)| { val = val.saturating_sub(*r0); (idx, val)}).collect::<HashMap<usize, u8>>(); //underflow here
    

    //indices_to_rmv.sort();
    indices_to_rmv.reverse();

    indices_to_rmv.iter().for_each(|x| {
        green.insert(**x, 255_u8);
        red.insert(**x, 255_u8);
    });
    


    let data = green.into_iter()
                    .zip(red.into_iter())
                    .map(|(g, r)| vec![g, r])
                    .collect::<Vec<Vec<u8>>>();

    Ok(data)

}
// Converts hashmap to vector of u8 while transposing the data
/*pub fn decompose(data: HashMap<usize, HashMap<usize, (u8, u8)>>) -> Vec<u8> {
    
    let sub_hash_len = data.get(&0).unwrap().len();            
    let data_to_vec = (0..sub_hash_len).into_iter()
                                       .map(|snp_idx| {
                                            (0..data.len()).into_iter().map(|indiv_idx| {
                                                let (green, red) = data.get(&indiv_idx)
                                                                       .unwrap()
                                                                       .get(&snp_idx)
                                                                       .unwrap();
                                                vec![*green, *red]
                                            })
                                                                       .flatten()
                                                                       .collect::<Vec<u8>>()
                                        })
                                        .flatten()
                                        .collect::<Vec<u8>>();
                                     
    data_to_vec
}*/
/*pub fn hash_to_vec(data: HashMap<usize, HashMap<usize, (u8, u8)>>) -> Vec<u8> {
    
    let sub_hash_len = data.get(&0).unwrap().len();  
    //indiv_idx
    //snp_idx  
    let data_to_vec = (0..data.len()).into_par_iter()
                                       .map(|indiv_idx| {
                                            (0..sub_hash_len).into_par_iter().map(|snp_idx| {
                                                let (green, red) = data.get(&indiv_idx)
                                                                       .unwrap()
                                                                       .get(&snp_idx)
                                                                       .unwrap();
                                                vec![*green, *red]
                                            })
                                                                       .flatten()
                                                                       .collect::<Vec<u8>>()
                                        })
                                        .flatten()
                                        .collect::<Vec<u8>>();
                                     
    data_to_vec
}*/

pub fn vec_transpose(data: Vec<u8>, no_snps: usize) -> Vec<u8> {
    let data = data.chunks_exact(2).map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();
    let data = data.chunks_exact(no_snps).map(|x| x.to_vec()).collect::<Vec<Vec<Vec<u8>>>>();

    let trans_vec = (0..no_snps).into_iter()
        .flat_map(|snp_idx| {
            data.iter().map(move |all_indiv_snps| {
                all_indiv_snps[snp_idx].clone()
            })
        })
        .flatten()
        .collect::<Vec<u8>>();
    
    trans_vec
}

pub fn normalise_data_from_file_chunks(chunks: Vec<Vec<(PathBuf, PathBuf)>>, rank: i32) -> Vec<Vec<Vec<u8>>> {
    chunks[rank as usize].iter()
                         //.enumerate()
                         .map(|green_red_paths| {
                             normalise_sample(&green_red_paths).unwrap()
                         })
                         .collect::<Vec<Vec<Vec<u8>>>>()     
}
