use std::fs::{self, OpenOptions, File};
use std::env;
use std::io::Write;


// fn print_contents(contents: &str, starting_line: u32, lines_number: u8) {
//     for (i,_line) in contents.split("\n").enumerate() {
//         println!("{i} {_line}")
//     }
// }

fn print_contents(contents: &str) {
    for (i,l) in contents.split("\n").enumerate() {
        println!("{} {}", i, l);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {panic!("Wrong input format. Correct input format has the filename as the only arg.")}
    let path = &args[1];
    let exists = fs::metadata(path).is_ok();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(!exists)
        .open(path)
        .expect("Unable to open file");

    let contents = fs::read_to_string(path).expect("Unable to read file");
    // file.write_all(b"gay").expect("Unable to write to file");
    print_contents(&contents);


    if !exists {
        println!("File created!")
    }
}
