
use std::path::{PathBuf};
use std::error;
use std::collections::{HashMap};

use rayon::prelude::*;

pub fn get_min_max<T: PartialOrd + Copy>(value_map: &HashMap<usize, T>, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut indices: Vec<usize> = value_map.keys().copied().collect();
    indices.sort_by(|&a, &b| {
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
}

pub fn normalise_sample(path: &(PathBuf, PathBuf)) -> Result<HashMap<usize, (u8, u8)>, Box<dyn error::Error>>  {
    // read file to vector
    let red = std::fs::read(&path.1).expect("Error reading red for indiv...");
    let green = std::fs::read(&path.0).expect("Error reading green for indiv...");

    //println!("\n");
    //println!("Green len: {:#?}", green.len());
    //println!("{:#?}", path);
    //println!("\n");
    //green.iter().enumerate().for_each(|(idx, val)| print!("({:?}: {:?}) ", idx, val)); //correct

    assert_eq!(green.len(), red.len());

    let ratio = green.par_iter()
                     .zip(red.par_iter())
                     .map(|(g, r)| *r as f32 / *g as f32)
                     .collect::<Vec<f32>>();
    // Convert vectors to HashMap for easier indexing
    let mut green = green.into_par_iter().enumerate().collect::<HashMap<usize, u8>>();
    let mut red = red.into_par_iter().enumerate().collect::<HashMap<usize, u8>>();
    let ratio = ratio.par_iter().enumerate().map(|(idx, val)| (idx, *val)).collect::<HashMap<usize, f32>>();
    //println!("\n");
    //(0..ratio.len()).for_each(|x| {
    //    println!("{:#?}: {:#?}", x, ratio.get(&x).unwrap()); //correct
    //});

    // get 5 smalles and largest values of red, green
    let green_min_max = get_min_max(&green, 5).unwrap();
    let red_min_max = get_min_max(&red, 5).unwrap();
    let ratio_min_max = get_min_max(&ratio, 5).unwrap();
    
    //println!("\n");
    //println!("ratios: {:#?}", ratio_min_max); //correct
    
    //println!("\n");
    //println!("{:#?}", green_min_max); //correct

    // put all indices to be removed into a vector
    let mut indices_to_rmv = green_min_max.par_iter()
                                          .chain(red_min_max.par_iter())
                                          .chain(ratio_min_max.par_iter())
                                          .collect::<Vec<&usize>>();

    //println!("\n");
    //println!("{:#?}", indices_to_rmv); //correct
    // small check to vverify that there are 20 values in total 
    assert_eq!(indices_to_rmv.len(), 30);

    indices_to_rmv.dedup();

    //println!("\n");
    //println!("{:?}", indices_to_rmv);

    // INTRODUCE Arc<Rc> HERE
    indices_to_rmv.iter()
                  .for_each(|x| {
                      green.remove(x);
                      red.remove(x);
                  });
    //println!("Green HashMap Capacity: {:?}", green.capacity());
    assert_eq!(green.len(), red.len());

    //println!("\n");
    //println!("{:#?}", green); //correct

    // indices not required to know the min max values
    let r0 = *red.values().min().unwrap();
    let g0 = *green.values().min().unwrap();
    
    //println!("\n");
    //println!("r0: {:?} g0: {:?}", r0, g0); //correct

    // do you subtact r0 from red and g0 from green ?
    let mut green = green.into_par_iter().map(|(idx, mut val)| { val = val.saturating_sub(g0); (idx, val)}).collect::<HashMap<usize, u8>>(); //underflow here
    let mut red = red.into_par_iter().map(|(idx, mut val)| { val = val.saturating_sub(r0); (idx, val)}).collect::<HashMap<usize, u8>>(); //underflow here
    //red.values_mut().for_each(|val| { *val = val.saturating_sub(r0); }); //underflow here    
    
    //println!("\n");
    //println!("{:#?}", green); //correct
    
    // add removed indices back again with value 255
    indices_to_rmv.into_iter()
                  .for_each(|x| {
                      green.insert(*x, 255_u8);
                      red.insert(*x, 255_u8);
                  });


    let data = green.into_par_iter().map(|(idx, value)| {
                                    (idx, (value, *red.get(&idx)
                                        .unwrap_or_else(|| panic!("red HashMap is None at idx: {:?} value: {:?}", idx, value))))        
                                }).collect::<HashMap<usize, (u8, u8)>>();

    //println!("\n");
    //println!("{:#?}", data); //correct
    Ok(data)

}
// Converts hashmap to vector of u8 while transposing the data
pub fn decompose(data: HashMap<usize, HashMap<usize, (u8, u8)>>) -> Vec<u8> {
    
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
}
pub fn hash_to_vec(data: HashMap<usize, HashMap<usize, (u8, u8)>>) -> Vec<u8> {
    
    let sub_hash_len = data.get(&0).unwrap().len();  
    //indiv_idx
    //snp_idx  
    let data_to_vec = (0..data.len()).into_iter()
                                       .map(|indiv_idx| {
                                            (0..sub_hash_len).into_iter().map(|snp_idx| {
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
}

pub fn vec_transpose(data: Vec<u8>, no_snps: usize) -> Vec<u8> {
    let data = data.chunks(2).map(|x| x.to_vec()).collect::<Vec<Vec<u8>>>();
    let data = data.chunks_exact(no_snps).map(|x| x.to_vec()).collect::<Vec<Vec<Vec<u8>>>>();

    let trans_vec = (0..no_snps)
        .flat_map(|snp_idx| {
            data.iter().map(move |all_indiv_snps| {
                all_indiv_snps[snp_idx].clone()
            })
        })
        .flatten()
        .collect::<Vec<u8>>();
    
    trans_vec
}

pub fn normalise_data_from_file_chunks(chunks: Vec<&[(PathBuf, PathBuf)]>, rank: i32) -> HashMap<usize, HashMap<usize, (u8, u8)>> {
    chunks[rank as usize].into_iter()
                         .enumerate()
                         .map(|(idx, green_red_paths)| {
                             (idx, normalise_sample(&green_red_paths).unwrap())
                         })
                         .collect::<HashMap<usize, HashMap<usize, (u8, u8)>>>()     
}