extern crate ring;

use crate::ring::rand::SecureRandom;
use ring::rand::SystemRandom;

fn main() -> Result<(), ring::error::Unspecified> {
    // generate random bytes
    let mut b1 = vec![0; 12];
    SystemRandom::new().fill(&mut b1)?;
    println!("{:?}", b1);

    let mut b2 = vec![0; 2];
    SystemRandom::new().fill(&mut b2)?;
    println!("{:?}", b2);

    Ok(())
}
