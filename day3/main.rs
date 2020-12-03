use std::fs;

fn main() {
	// Read file
	let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file!");

	// Initialize tree field
	let map = Map::new(contents);

	let right1down1 = tree_finder(1, 1, 1, &map);
	let right3down1 = tree_finder(3, 1, 1, &map); // part 1
	let right5down1 = tree_finder(5, 1, 1, &map);
	let right7down1 = tree_finder(7, 1, 1, &map);
	let right1down2 = tree_finder(1, 2, 2, &map);

	let total_trees: i64 = right1down1*right3down1*right5down1*right7down1*right1down2;
	println!("{:?}", total_trees );
}

fn tree_finder(right: usize, down: usize, start:usize, map: &Map) -> i64 {
	let mut trees_amount: i64 = 0;
	let mut current_column: usize = 0;
	for current_height in (start..map.height()).step_by(down) {
		current_column += right;
		if map.tree_at(current_column, current_height) {
			trees_amount += 1;
		}
	}
	return trees_amount;
}


struct Map {
	rows: Vec<String>,
}

impl Map {
	fn new(starting_map: String) -> Map {
		let mut map_rows: Vec<String> = Vec::<String>::new();
		
		for row in starting_map.split("\n") {
			if row != "" {
				map_rows.push(row.to_string());
			}
		}

		return Map { rows:map_rows }
	}

	fn height(&self) -> usize {
		return self.rows.len();
	}

	fn tree_at(&self, column: usize, height: usize) -> bool {
		let true_column: usize = column % self.rows[0].len();
		let row_as_chars = self.rows[height].chars().collect::<Vec<_>>();
		return row_as_chars[true_column] == '#';
	}


}