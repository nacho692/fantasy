use crate::game::character::Character;

pub struct Encounter<'a> {
	grid: Vec<Vec<&'a Placeable>>,
	characters: Vec<&'a Character>
}

pub fn new<'a>(grid_width: usize, grid_height: usize) -> Encounter<'a> {
	let mut grid: Vec<Vec<&Placeable>> = Vec::new();
	for _ in 1..=grid_width {
		let mut col: Vec<&Placeable> = Vec::new();

		for _ in 1..=grid_height {
			col.push(&EmptySpace{});
		}

		grid.push(col);
    }
	return Encounter {
		grid: grid,
		characters: Vec::new()
	};
}

impl<'a> Encounter<'a> {
	pub fn new_character(&mut self, character: &'a Character, x_position: usize, y_position: usize) {
		self.grid[x_position-1][y_position-1] = character;
		self.characters.push(character)
	}
}


pub trait Placeable {}

struct EmptySpace {}

impl Placeable for EmptySpace {}

