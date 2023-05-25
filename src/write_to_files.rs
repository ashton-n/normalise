use std::collections::HashMap;
use std::fs;
use std::io::{LineWriter, Write};
use std::io;
use std::path::{Path, PathBuf};

pub fn write_to_bin(
    write_location: &PathBuf,
    data: HashMap<usize, HashMap<usize, (u8, u8)>>,
) -> io::Result<String> {
    
    // Needs to be done like this
    //let file_path = std::env::current_dir().unwrap();

    // creates a file
    //let file = std::fs::File::create("./test").unwrap();

    let mut file = fs::OpenOptions::new().write(true)
                                         .create(true)
                                         .open(write_location)
                                         .expect("output file path incorrect");
    
    //(0..data.len()).for_each(|sample_idx| {
    //    let (green, red) = *(data.get(&indiv_idx).unwrap()).get(&sample_idx).unwrap();
        //data.iter().for_each(|(_snp_idx, snp_pair)|{
        //    let (green, red) = *snp_pair.get(&sample_idx).unwrap();
    //        file.write(&[green, red]).unwrap();
        //    //println!("{:?} {:?}",green, red);

        //});
    //});

    data.iter().for_each(|(indiv_idx, indiv_rg_pairs)| {
                    println!("\nidx: {:?}", indiv_idx);
                    //indiv_rg_pairs.iter().for_each(|(snp_idx, snp_pairs)| {
                        (0..indiv_rg_pairs.len()).for_each(|x| {
                            let (green, red) =  indiv_rg_pairs.get(&x).unwrap();
                            println!("{:?} {:?} {:?}", x, green, red);
                            file.write(&[*green, *red]).unwrap();
                            }
                        );
                    //})
                });
    
    
    file.flush()?;
    Ok(String::from(format!("Write Complete.")))
}