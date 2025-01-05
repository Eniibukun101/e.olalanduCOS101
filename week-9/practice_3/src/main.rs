use std::fs;

fn main() {
    fs::remove_file("mean.txt").expect("could not remove file");
    println!("The file has been removed!");
}
