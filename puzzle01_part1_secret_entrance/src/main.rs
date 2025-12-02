use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut dial_location: i32 = 50;
    let mut count_zeros: u32 = 0;
    let modulo: i32 = 100;
    let file_path: &str = "input/input";

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let replace_r = line.replace('R', "");
            let replace_l = replace_r.replace('L', "-");
            let movement: i32 = replace_l.parse().unwrap();
            
            dial_location = (dial_location + movement).rem_euclid( modulo );
            if dial_location == 0
            {
                count_zeros += 1;
            }
            println!("{} {} {}", line, dial_location, count_zeros);
        }
    }

    println!("Num Zeros: {}", count_zeros);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
