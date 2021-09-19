use find::list_files;
use std::io;
use std::fs::{self};

/*

-p : path from where to search, if not provided then searching form current
-s : is substring, if not provided then searching whole string
-d : depth search (default)
-w : wide search

Usage:
fd query -p path/path1
fd query -s

*/


fn main()  -> io::Result<()>{
    list_files();

    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    // The order in which `read_dir` returns entries is not guaranteed. If reproducible
    // ordering is required the entries should be explicitly sorted.

    entries.sort();

    println!("{:?}", entries);
    Ok(())
}
