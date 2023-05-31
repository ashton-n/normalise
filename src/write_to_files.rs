use std::fs::{OpenOptions};
use std::io::{Write, BufWriter};
use std::io;
use std::path::{PathBuf};



/*pub fn write_hashmap_to_bin(
    write_location: &PathBuf,
    data: HashMap<usize, HashMap<usize, (u8, u8)>>,
) -> io::Result<String> {
    let mut file = fs::OpenOptions::new().write(true)
                                         .create(true)
                                         .open(write_location)
                                         .expect("output file path incorrect");
    
    (0..data.len()).for_each(|indiv_idx| { 
        
        let indiv_rg_pairs = data.get(&indiv_idx).unwrap(); 
        println!("\nidx: {:?}", indiv_idx + 1);
        (0..indiv_rg_pairs.len()).for_each(|x| {
            let (green, red) =  indiv_rg_pairs.get(&x).unwrap();
            println!("{:?} {:?} {:?}", x + 1, green, red);
            file.write(&[*green, *red]).unwrap();
            }
        );
    });
    
    
    file.flush()?;
    Ok(String::from(format!("Write Complete.")))
}*/

/*pub fn write_vec_to_bin(
    write_location: &PathBuf,
    data: Vec<u8>,
) -> io::Result<String> {

    let mut file = fs::OpenOptions::new().write(true)
                                         .create(true)
                                         .open(write_location)
                                         .expect("output file path incorrect");
    
    data.into_iter().for_each(|x| {
        file.write(&[x]).unwrap();
    });
    
    file.flush()?;
    Ok(String::from(format!("Write Complete.")))
}*/

/*pub fn write_vec_to_bin(write_location: &PathBuf, data: Vec<u8>) -> io::Result<String> {
    let file = Mutex::new(BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(write_location)
            .expect("output file path incorrect")
    ));

    data.into_par_iter().for_each(|x| {
        let mut locked_file = file.lock().unwrap();
        locked_file.write(&[x]).unwrap();
    });

    let mut locked_file = file.lock().unwrap();
    locked_file.flush()?;

    Ok(String::from("Write Complete."))
}*/

pub fn write_vec_to_bin(write_location: &PathBuf, data: Vec<u8>) -> io::Result<String> {
    let mut file = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(write_location)
            .expect("output file path incorrect")
    );

    data.into_iter().for_each(|x| {
        //let mut locked_file = file.lock().unwrap();
        file.write(&[x]).unwrap();
    });

    //let mut locked_file = file.lock().unwrap();
    file.flush()?;

    Ok(String::from("Write Complete."))
}
