use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    // if a < b {
    if a < b as i32 {
        println!("Ten is less than one hundred.");
    }

    if a < b.try_into().unwrap() {
        println!("Ten is less than one hundred.");
    }

    println!("{}", 0.1 + 0.2);
    // assert!(0.1 + 0.2 == 0.3);

    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
}
