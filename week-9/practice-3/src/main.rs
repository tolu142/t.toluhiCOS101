use std::fs;

fn main() {
   fs::remove_file("data.txt").expect("could remove file");
   println!("file is removed");

}
