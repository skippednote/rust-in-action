use std::{error::Error, fs::File, net::Ipv6Addr};

// fn main() -> Result<(), std::io::Error> {
fn main() -> Result<(), Box<dyn Error>> {
    // let _f = File::open("invisible.txt")?;
    // Ok(())
    let _f = File::open("invisible.txt")?;
    let _localhost = "::1".parse::<Ipv6Addr>()?;
    Ok(())
}
