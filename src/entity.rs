#[path = "item.rs"] mod items;

pub struct Enemy {
    hp: i16,
    name: String,
    weapon: items::Weapon,
}

impl Enemy {
    pub fn new(hp: i16, name: String, weapon: items::Weapon) {

    } 
}