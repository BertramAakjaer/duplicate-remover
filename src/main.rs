use std::{fs, path::PathBuf};

fn main() {
    let cur_dir: fs::ReadDir = get_input_string("Enter directory for files to compare:");
    let dump_dir: fs::ReadDir = get_input_string("Enter directory for duplicate dump:");

    // println!("The path is {:?}", path);
    println!("current directory: {:?} \ndump directory: {:?}", cur_dir, dump_dir);

    for path in cur_dir {
        println!("Name: {}", path.unwrap().path().display())
    }
}

/*

    Functions used within the script

*/

fn get_input_string (print_string: &str) ->  fs::ReadDir {
    println!("{}", print_string);

    let mut rtn_string: String = String::new();
    std::io::stdin()
                    .read_line(&mut rtn_string)
                    .expect("Error type not reconised");

    let path: PathBuf = rtn_string.trim().into();
    let paths: fs::ReadDir = fs::read_dir(&path).expect("Path doesnt excist :'(");

    println!("\n");

    paths
}