use std::fs;

fn main(){
    let mut path = String::new();

    path.push_str("/proc/stat");
    let file = fs::read_to_string(path)
        .expect("Permission denied");
    for i in 0..16{
    let mut counter = 0;
    loop{

    }
        for byte in file.split_whitespace() {
            println!("Content : {}", byte);
            let value: f64 = byte.trim().parse().expect("Parse error");
            for n in 1..100{
                if n > value as i32 {
                    print!("\x1b[96m|\x1b[0m");
                } else {
                    print!("\x1b[92m|\x1b[0m");
                }
            }
            break ;
        }
        println!("\n");
    }
}
