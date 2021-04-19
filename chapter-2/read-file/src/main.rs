use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("Cargo.toml").unwrap();
    let reader = BufReader::new(f);
    // let mut line = String::new();

    // loop {
    //     let len = reader.read_line(&mut line).unwrap();
    //     if len == 0 {
    //         break;
    //     }

    //     println!("{} ({} bytes long)", line.trim(), len);
    //     line.truncate(0);
    // }

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
