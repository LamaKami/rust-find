use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::path::PathBuf;
use std::fs::metadata;

pub fn list_files(){
    // unwrap könnte fehl schlagen wenn keine Rechte vorhanden sind
    let paths = fs::read_dir("./").unwrap();

    for path in paths{
        println!("{}", path.as_ref().unwrap().path().display());
        
        check_if_directory_or_files(path.as_ref().unwrap().path());

        check_if_directory_or_file(path.unwrap().path().into_os_string().into_string().unwrap());
    }
}

fn check_if_directory_or_files(path: PathBuf){
    println!("{}", path.is_dir());
    println!("{}", path.is_file());

}

fn check_if_directory_or_file(path: String) {
    let md = metadata(path).unwrap();
    println!("{}", md.is_dir());
    println!("{}", md.is_file());

}

pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}