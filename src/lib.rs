use std::{process};
use std::fs::{self, DirEntry, ReadDir};
use std::path::PathBuf;
use std::error::Error;

pub fn search_for_file(query: String, path: String, is_substring: bool){
    if is_substring {
        search_for_substring(query, path)
    }
    else {
        search_for_string(query, path)
    }
}

fn search_for_substring(query: String, path: String){
    for entry in get_directory_entries(path){
        if entry.as_ref().unwrap().path().is_dir(){
            search_for_substring(query.clone(), entry.as_ref().unwrap().path().to_str().unwrap().to_owned());
        }
        else if entry.as_ref().unwrap().file_name().to_str().unwrap().contains(&query){
            println!("{}", entry.as_ref().unwrap().path().display());
        }
    }
}

fn search_for_string(query: String, path: String){
    for entry in get_directory_entries(path){
        if entry.as_ref().unwrap().path().is_dir(){
            search_for_string(query.clone(), entry.as_ref().unwrap().path().to_str().unwrap().to_owned());
        }
        else if entry.as_ref().unwrap().file_name().to_str().unwrap().eq(&query) {
            println!("{}", entry.as_ref().unwrap().path().display());
        }
    }
}

fn get_directory_entries(path: String) -> ReadDir{
    return fs::read_dir(&path).unwrap_or_else(|err| {
        eprintln!("Reading path produced Error: {}",err);
        process::exit(1);
    });
}

pub fn list_files(){
    // unwrap könnte fehl schlagen wenn keine Rechte vorhanden sind
    let paths = fs::read_dir("./").unwrap();

    for path in paths{
        println!("{}", path.as_ref().unwrap().path().display());
        
        check_if_directory_or_files(path.as_ref().unwrap().path());
    }
}


fn check_if_directory_or_files(path: PathBuf){
    println!("{}", path.is_dir());
    println!("{}", path.is_file());
}





