use rppal::i2c::I2c;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut port = 0;
    match args.get(1) {
        Some(x) => port = x.parse().unwrap(),
        None => port = 13,
    }
    println!("I2c scanner port {}", port);
    let mut reg = [0u8; 3];
    let mut i2c = I2c::with_bus(port)?;
    println!("     00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F");
    print!("00 :                         ");
    for addr in 8..0x77 {
        i2c.set_slave_address(addr)?;
        let res = i2c.read(&mut reg);
        if (addr % 16) == 0 {
            print!("\n{:1x} : ", (addr / 16) * 16);
        }
        match res {
            Ok(_) => print!("{:2x} ", addr as u8),
            Err(_) => print!("-- "),
        }
    }
    Ok(())
}
