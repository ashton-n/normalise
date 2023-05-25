use std::ffi::OsStr;
//use std::io::{BufRead, Error, Result, self};
use std::io;
use std::error;
use std::fs::{read_dir, ReadDir};
use std::path::{Path, PathBuf};
use std::collections::HashMap;


pub fn get_file_pathes(input_dir: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {    
    let entries = read_dir(input_dir).unwrap();
    
    let paths = entries.map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>();

    let mut data_paths = paths.into_iter()
                              .filter(|path| path.extension() == Some(OsStr::new("dat")))
                              .filter(|path| path
                                .file_stem()
                                .and_then(|stem| stem.to_str())
                                .and_then(|str| str.chars().last())
                                .map(|last_char| last_char == 'n' || last_char == 'd').unwrap())
        
                              .collect::<Vec<PathBuf>>();

    let (green, red): (Vec<_>, Vec<_>) = data_paths.into_iter()
                                                   .partition(|path| path
                                                        .file_stem()
                                                        .and_then(|stem| stem.to_str())
                                                        .and_then(|str| str.chars().last())
                                                        .map(|last_char| last_char == 'n')
                                                        .unwrap_or(false)
                                                    );                                                

    let sample_pair_paths = green.into_iter()
                                 .zip(red.into_iter())
                                 .collect::<Vec<(_, _)>>();
    Ok(sample_pair_paths)
}

 
// stores the contents of a file in a vector
pub fn read_file_to_hashmap(
    data_source: &Path,
) -> Result<HashMap<usize, u8>, Box<dyn error::Error>> {

    //println!("[DEBUG] file_path is: {:?}", file_path); //debugging output
    let data = std::fs::read(data_source)?;
    let data = data.into_iter().enumerate().collect::<HashMap<usize, u8>>();

    Ok(data)
}
/*
//counts the number of lines in a file
pub fn count_lines(
    data_source: &str,
    file_name: &str,
    extension: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    // generates path and name of file to be read
    let mut file_path = String::from(data_source);
    file_path.push_str(file_name);
    file_path.push_str(extension);
    // create file handle
    let file_handle = std::io::BufReader::new(std::fs::File::open(&file_path)?);
    let mut count: usize = 0;
    // count lines
    for _ in file_handle.lines() {
        count += 1;
    }
    Ok(count)
}

// Gets CHR, SNP and POS from the .bim file
pub fn get_bim_info(
    data_source: &str,
    file_name: &str,
) -> Result<(Vec<String>, Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    // generates path and name of file to be read
    let mut file_path = String::from(data_source);
    file_path.push_str(file_name);
    file_path.push_str(".bim");
    // create file handle
    let file_handle = std::io::BufReader::new(std::fs::File::open(&file_path)?);
    // assign vars to store data in
    let mut chr = Vec::new();
    let mut snp = Vec::new();
    let mut pos = Vec::new();

    for line in file_handle.lines() {
        // split each line into words
        for (idx, word) in line.unwrap().split("\t").enumerate() {
            // match index to figure out where data needs to be stored
            match idx {
                0 => chr.push(word.to_string()),
                1 => snp.push(word.to_string()),
                2 => pos.push(word.to_string()),
                _ => {}
            }
        }
    }
    Ok((chr, snp, pos))
}

// reads .fam file to get the number of cases/controls
// and the case/control assignments
pub fn get_control_case(
    data_source: &str,
    file_name: &str,
    extension: &str,
    no_indivs: &usize,
) -> Result<(usize, usize, Vec<u8>), Box<dyn std::error::Error>> {
    // generates path and name of file to be read
    let mut file_path = String::from(data_source);
    file_path.push_str(file_name);
    file_path.push_str(extension);

    let buf_reader = std::io::BufReader::new(std::fs::File::open(&file_path)?);
    // assign collections for data
    let mut controls = 0;
    let mut cases = 0;
    // assign default as 2 to indicate cases
    let mut control_case: Vec<u8> = vec![2; *no_indivs];

    buf_reader.lines().enumerate().for_each(|(line_idx, line)| {
        // split the line into words, keeping the last element,
        // using it to count cases/controls and store assignments
        match line
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
        {
            &"1" => {
                controls += 1;
                control_case[line_idx] = 1;
            }
            &"2" => cases += 1,
            _ => println!("oops"),
        }
    });

    Ok((controls, cases, control_case))
}
// unit test which uses small.fam to validate functionality
#[test]
fn test_get_control_case() {
    println!("{}", std::env::current_dir().unwrap().display());
    let (controls, cases, control_case_idx) =
        get_control_case(&"src/data/", &"small", &".fam", &10).unwrap();
    assert_eq!(
        (controls, cases, control_case_idx),
        (0 as usize, 10 as usize, vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2])
    )
}
*/
