use std::collections::HashMap;

fn main() {
	let data: Vec<&str> = include_str!("../3.txt").lines().collect();
	let width = data[0].len();
	let mut engine_parts: Vec<u32> = Vec::new();
	let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

	for row in 0..data.len() {
		let mut part_number = String::new();
		let mut adjacent_gear_positions = Vec::new();
		let mut is_adjacent = false;
		for col in 0..width {
			let char = data[row].chars().nth(col).unwrap();
			if char.is_numeric() {
				part_number.push(char);
				for y in -1..=1isize {
					let row = row as isize + y;
					if row < 0 || row == data.len() as isize {
						continue;
					}
					let row = row as usize;
					for x in -1..=1isize {
						let col = col as isize + x;
						if col < 0 || col >= width as isize {
							continue;
						}
						let col = col as usize;
						let char = data[row].chars().nth(col).unwrap();
						if !char.is_numeric() && char != '.' {
							is_adjacent = true;
						}
						if char == '*' {
							adjacent_gear_positions.push((row, col));
						}
					}
				}
			} else if !part_number.is_empty() {
				let part_number_u32 = part_number.parse().unwrap();
				if is_adjacent {
					engine_parts.push(part_number_u32);
				}
				adjacent_gear_positions.sort();
				adjacent_gear_positions.dedup();
				for pos in &adjacent_gear_positions {
					if let Some(gear) = gears.get_mut(pos) {
						gear.push(part_number_u32);
					} else {
						gears.insert(pos.clone(), vec![part_number_u32]);
					}
				}

				adjacent_gear_positions.clear();
				is_adjacent = false;
				part_number.clear();
			}
		}
		if !part_number.is_empty() && is_adjacent {
			let part_number_u32 = part_number.parse().unwrap();
			engine_parts.push(part_number_u32);

			adjacent_gear_positions.sort();
			adjacent_gear_positions.dedup();
			for pos in &adjacent_gear_positions {
				if let Some(gear) = gears.get_mut(pos) {
					gear.push(part_number_u32);
				} else {
					gears.insert(pos.clone(), vec![part_number_u32]);
				}
			}
		}
	}
	println!("{}", engine_parts.iter().sum::<u32>());
	let gear_sum: u32 = gears
		.values()
		.filter(|v| v.len() == 2)
		.map(|v| v[0] * v[1])
		.sum();
	println!("{}", gear_sum);
}
