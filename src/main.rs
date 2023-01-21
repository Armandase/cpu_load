use std::fs::{self};

fn main(){
    let mut path = String::new();

    path.push_str("/proc/stat");
    let content = fs::read_to_string(path)
        .expect("Permission denied");
    for line in content.lines(){
        let mut index = 0;
        let mut number: f32 = 0.0;
        let mut user: f32 = 0.0;

        for byte in line.split_whitespace() {
            if index == 0{
                if byte.as_bytes()[0] != b'c'{
                    return ;
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
        for n in 1..100{
            if n > load as i32 {
                print!("\x1b[37m▄\x1b[0m");
            } else {
                print!("\x1b[31m▄\x1b[0m");
            }
        }
        println!(" -> {:.2}%\n", load);
    }
}
