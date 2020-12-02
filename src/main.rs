extern crate getopts;
use getopts::Options;
use std::env;

fn usage(binary: &str, opts: Options) {
    let output_usage = format!("Usage: {} [options] DIR ", binary);
    print!("{}", opts.usage(&output_usage));
}

fn is_dir(path: &str) -> std::io::Result<()> {
    let check = std::fs::metadata(&path).unwrap();
    if !check.is_dir() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other,"Is Directory"))
    }
}

fn is_symlink(path: &str) -> std::io::Result<()> {
    let metadata = std::fs::symlink_metadata(&path)?;
    let check = metadata.file_type();
    if !check.is_symlink() {
       Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other,"Is symlink"))
    }
}

fn files(directory: &str, function_array: &Vec<fn(&str) -> std::io::Result<()>>) -> Result<(), Box<dyn std::error::Error>>{
    let mut counter = 0;

    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path().display().to_string();
        if !function_array.is_empty() {
            for function in function_array{
                match function(&path) {
                    Ok(_) =>  counter += 1, 
                    Err(_) => ()
                }
            }
        } else { 
           counter += 1;
        }
    }

    print!("{}: {}\n", &directory, &counter);

    Ok(())
}

fn files_recursive(directory: &str) -> Vec<String>{

    let mut dirs : Vec<String> = Vec::new();
    for dir in walkdir::WalkDir::new(directory){
        let dir = dir.unwrap().path().display().to_string();
        if std::fs::metadata(&dir).unwrap().is_dir() {
            dirs.push(dir);
        }
    }
    return dirs;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let binary = args[0].clone();
    let mut function_array: Vec<fn(&str) -> std::io::Result<()>> = Vec::new();


    let mut opts = Options::new();
    opts.optflag("d", "directory", "don't count directory");
    opts.optflag("s", "symlink", "don't count symlink");
    opts.optflag("r", "recursive", "print recursive");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&binary, opts);
        return;
    }
    let directory = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        "./".to_string()
    };

    if matches.opt_present("d") {
        let name : fn(&str) -> std::io::Result<()> = is_dir ;
        function_array.push(name);
    }

    if matches.opt_present("s") {
        let name : fn(&str) -> std::io::Result<()> = is_symlink ;
        function_array.push(name);
    }

    if matches.opt_present("r") {
        for dir in files_recursive(&directory) {
            match files(&dir, &function_array) {
                Err(e) => println!("{:?}", e),
                _ => ()
            }
       }

    } else {

        match files(&directory, &function_array) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }

}
