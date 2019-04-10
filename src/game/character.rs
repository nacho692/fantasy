use crate::game::encounter::Placeable;
use crate::game::encounter::Encounter;

pub struct Character {
	name: String,
	health: u32,
	damage: u32
}

pub fn new(name: String, health: u32, damage: u32, an_encounter: &mut Encounter, x_position: usize, y_position: usize) -> Character {
	let character = Character {
		name: name,
		health: health,
		damage: damage
	};
	an_encounter.new_character(character, x_position, y_position);
	return character;
}


impl Placeable for Character {

}