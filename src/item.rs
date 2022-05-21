use std::fs::File;
use std::io::{BufReader, BufRead};


pub struct Weapon {
    attack: i16,
    name: String,
}

pub fn get_weapons_from_file(filename: String) -> Vec<String> {
    let file = File::open(filename).expect("Issue opening file");
    let buffer = BufReader::new(&file);

    let lines: Vec<String> = buffer.lines().collect::<Result<_,_>>().unwrap();

    lines
}

pub fn get_weapon_from_string(weapon_string: String) -> Weapon {
    let split_weapon_string = weapon_string.split(" ").collect::<Vec<&str>>();
    
    let weapon_name = split_weapon_string[0].to_string();
    let attack = split_weapon_string[1].parse::<i16>().unwrap();

    
    // println!("Weapon name: {} Attack: {}", weapon_name, attack);

    Weapon {
        name: weapon_name,
        attack: attack,
    }

}

impl Weapon {

}

pub struct Armor {
    defense: i16,
    name: String,
}