fn main() {
    let mut i: u16 = 0;
    println!("{}", i);

    loop {
        i += 1000;
        println!("{}", i);
        if i % 10000 == 0 {
            print!("\n")
        }
    }
}
