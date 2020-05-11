// FILE READ
use std::fs::File;
use std::io::Read;
use std::io;

// SERDE SERIALIZATION (YAML/JSON)
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod dice_parent;
use dice_parent::DiceParent;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("data.yml")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() -> Result<(), serde_yaml::Error> {
    let s = read_username_from_file().unwrap();
    let dice_parent: DiceParent = serde_yaml::from_str(&s)?;
    println!("Dice parent property val1 = {}", dice_parent.val1.unwrap());
    Ok(())
}
