
use std::path::{Path, PathBuf};
use std::error;
use std::collections::HashMap;

use rayon::prelude::*;

use crate::read_from_files::read_file_to_hashmap;



/*pub fn get_min_max(data: &Vec<u8>, n: usize) -> Result<Vec<u8>, Box<dyn error::Error>> {

    let mut data = data.clone();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut min = data[0..n].to_vec();
    let max = data[(data.len()-n)..].to_vec();
    min.extend(max);
    Ok(min)

}*/

/*pub fn get_min_max_f32(data: &Vec<f32>, n: usize) -> Result<Vec<f32>, Box<dyn error::Error>> {

    let mut data = data.clone();

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut min = data[0..n].to_vec();
    let max = data[(data.len()-n)..].to_vec();
    min.extend(max);
    Ok(min)

}*/

pub fn get_min_max_u8(value_map: &HashMap<usize, u8>, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut indices: Vec<usize> = value_map.keys().copied().collect();
    indices.sort_by(|&a, &b| value_map[&a].cmp(&value_map[&b]));
    let mut idxs = indices[0..n].to_vec();
    let max = indices[(indices.len()-n)..].to_vec();
    idxs.extend(max);
    Ok(idxs)
}

pub fn get_min_max_f32(value_map: &HashMap<usize, f32>, n: usize) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut indices: Vec<usize> = value_map.keys().copied().collect();
    indices.sort_by(|&a, &b| {
        value_map[&a].partial_cmp(&value_map[&b]).unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut idxs = indices[0..n].to_vec();
    let max = indices[(indices.len()-n)..].to_vec();
    idxs.extend(max);
    Ok(idxs)
}

pub fn normalise_sample(path: &(PathBuf, PathBuf)) -> Result<HashMap<usize, (u8, u8)>, Box<dyn error::Error>>  {
    // read file to vector
    let red = std::fs::read(&path.1).expect("Error reading red for indiv...");
    let green = std::fs::read(&path.0).expect("Error reading green for indiv...");

    //green.iter().for_each(|x| print!("{:?} ", x)); //correct

    assert_eq!(green.len(), red.len());

    let ratio = green.iter()
                     .zip(red.iter())
                     .map(|(g, r)| *g as f32 / *r as f32)
                     .collect::<Vec<f32>>();
    // Convert vectors to HashMap for easier indexing
    let mut green = green.into_iter().enumerate().collect::<HashMap<usize, u8>>();
    //println!("\n");
    //println!("Green HashMap Capacity: {:?}", green.capacity());
    //println!("\n");
    //println!("{:#?}", green); //correct

    let mut red = red.into_iter().enumerate().collect::<HashMap<usize, u8>>();
    let mut ratio = ratio.iter().enumerate().map(|(idx, val)| (idx, *val)).collect::<HashMap<usize, f32>>();
    
    // get 5 smalles and largest values of red, green
    let green_min_max = get_min_max_u8(&green, 5).unwrap();
    let red_min_max = get_min_max_u8(&red, 5).unwrap();
    let ratio_min_max = get_min_max_f32(&ratio, 5).unwrap();
    //println!("\n");
    //println!("{:#?}", green_min_max); //correct

    // put all indices to be removed into a vector
    let mut indices_to_rmv = green_min_max.iter()
                                          .chain(red_min_max.iter())
                                          .chain(ratio_min_max.iter())
                                          .collect::<Vec<&usize>>();

    //println!("\n");
    //println!("{:#?}", indices_to_rmv); //correct
    // small check to vverify that there are 20 values in total 
    assert_eq!(indices_to_rmv.len(), 30);

    indices_to_rmv.dedup();

    //println!("\n");
    //println!("{:?}", indices_to_rmv);

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
    green.values_mut().for_each(|val| { *val = val.saturating_sub(g0); }); //underflow here
    red.values_mut().for_each(|val| { *val = val.saturating_sub(r0); }); //underflow here    
    
    //println!("\n");
    //println!("{:#?}", green); //correct
    
    // add removed indices back again with value 255
    indices_to_rmv.into_iter()
                  .for_each(|x| {
                      green.insert(*x, 255_u8);
                      red.insert(*x, 255_u8);
                  });

    let data = green.into_iter().map(|(idx, value)| {
                                    (idx, (value, *red.get(&idx)
                                        .unwrap_or_else(|| panic!("red HashMap is None at idx: {:?} value: {:?}", idx, value))))        
                                }).collect::<HashMap<usize, (u8, u8)>>();

    //println!("\n");
    //println!("{:#?}", data); //correct
    Ok(data)

}

pub fn transpose(data: HashMap<usize, HashMap<usize, (u8, u8)>>) 
    -> Result< Vec<Vec<(u8, u8)>>, Box<dyn error::Error> > {

    let transposed_data = (0..data.len()).into_iter()
                                         .map(|snp_idx| {
                                            //(data.get(&idx)).get(idx)
                                            data.iter().map(|(_sample_idx, norm_sample)| {
                                                *norm_sample.get(&snp_idx).unwrap()//.unwrap_or_else(|| panic!("error on normalised data at SNP {:?} value: {:?}", &snp_idx, norm_sample))
                                            }).collect::<Vec<(u8, u8)>>()  
                                         })
                                         .collect::<Vec<Vec<(u8, u8)>>>();

    Ok(transposed_data)
}

