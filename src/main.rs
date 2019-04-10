mod game;

use crate::game::character;
use crate::game::encounter;

fn main() {
	let mut an_encounter = encounter::new(32, 64);
	let hero = character::new(String::from("Jorge"), 50, 4, &mut an_encounter, 14, 13);
}
