use std::fs;

fn main(){
    let mut path = String::new();

    path.push_str("/proc/loadavg");
    let file = fs::read_to_string(path)
        .expect("Permission denied");
    println!("Content : {}", file);
    for byte in file.split_whitespace() {
        println!("Content : {}", byte);
        let mut value: f64 = byte.trim().parse().expect("Parse error");
        value *= 10.0;
        for n in 1..100{
            if n > value as i32 {
                print!("\x1b[96m|\x1b[0m");
            } else {
                print!("\x1b[92m|\x1b[0m");
            }
        }
        break ;
    }
}
