use std::ffi::OsStr;
use std::io;
use std::fs::{read_dir};
use std::path::{Path, PathBuf};

pub fn get_file_pathes(input_dir: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {    
    let entries = read_dir(input_dir).unwrap();
    
    let paths = entries.map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>();

    let mut data_paths = paths.into_iter()
                              .filter(|path| path.extension() == Some(OsStr::new("dat")))
                              .filter(|path| path.extension() != None)
                              .filter(|path| path
                                .file_stem()
                                .and_then(|stem| stem.to_str())
                                .and_then(|str| str.chars().last())
                                .map(|last_char| last_char == 'n' || last_char == 'd').unwrap())
        
                              .collect::<Vec<PathBuf>>();

    data_paths.sort();
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
