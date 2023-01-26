use std::fs::{self};
use std::thread::{sleep};
use std::time::{Duration};
use std::process::{Command};

fn  collect_calcul(line :&str) -> f32{
    let mut index = 0;
    let mut number: f32 = 0.0;
    let mut user: f32 = 0.0;

    for byte in line.split_whitespace() {
        if index == 0{
            if byte.as_bytes()[0] != b'c'{
                return -1.0;
            }
            print!("{} : ", byte);
            index += 1;
            continue ;
        };
        let value = byte.trim().parse().expect("Parsing error");
        number += value;
        if index == 4{
            user = value;
        }
        index += 1;
    }
    let load: f32 = 100.0 - (user / number) * 100.0;
    return load;
}

fn  print_load (load: f32){
    for n in 1..100{
        if n > load as i32 {
            print!("\x1b[37m▄\x1b[0m");
        } else {
            print!("\x1b[31m▄\x1b[0m");
        }
    }
    println!(" -> {:.2}%\n", load);
}

fn main(){
    let mut path = String::new();

    path.push_str("/proc/stat");
    loop{
        {
       let content = fs::read_to_string(&path)
            .expect("Permission denied");
        for line in content.lines(){
            let load: f32 = collect_calcul(line);
            if load == -1.0{
                break ;
            }
            print_load(load);
        }
        }
        sleep(Duration::from_millis(5000));
        Command::new("/usr/bin/clear")
            .spawn()
            .expect("Command fail");
        sleep(Duration::from_millis(5));
    }
}
