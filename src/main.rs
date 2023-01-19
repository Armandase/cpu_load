use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main(){
    let mut path = String::new();
    let mut counter = 0;
    let mut number: u64;

    path.push_str("/proc/stat");
    while let Ok(line) = read_lines(path){
        let mut index = 1;
        let mut user = 0;
        if counter == 0{
            counter += 1;
            continue ;
        }
        number = 0;
        let byte = line.split_whitespace();
        print!("{}", byte);
        for byte in line.split_whitespace() {
            let value = byte.trim().parse.expect("Parsing error");
            number += value;
            if index == 4{
                user = value;
            }
            index += 1;
        }
        let load = 100 - (user / number) * 100;
        println!("{}%", load);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
