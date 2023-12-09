use std::{collections::HashMap, fs};

use num::Integer;

fn main() {
	let data = fs::read_to_string("input.txt").unwrap();
	let (instructions, map) = data.split_once("\n\n").unwrap();
	let map: HashMap<String, (String, String)> = map
		.lines()
		.map(|line| {
			// BKM = (CDC, PSH)
			let source = line[..3].to_owned();
			let left = line[7..10].to_owned();
			let right = line[12..15].to_owned();
			(source, (left, right))
		})
		.collect();

	let mut location = "AAA";
	let mut steps = 0;
	while location != "ZZZ" {
		location = match &instructions[steps % instructions.len()..][..1] {
			"L" => &map[location].0,
			"R" => &map[location].1,
			_ => panic!(),
		};
		steps += 1;
	}
	println!("{steps}");

	let paths = map.keys().filter(|s| s.ends_with('A')).map(|start| {
		let mut location = start;
		let mut steps = 0;
		while !location.ends_with('Z') {
			location = match &instructions[steps % instructions.len()..][..1] {
				"L" => &map[location].0,
				"R" => &map[location].1,
				_ => panic!(),
			};
			steps += 1;
		}
		steps
	});
	let total_steps: usize = paths.fold(1, |a, b| a.lcm(&b));
	println!("{total_steps}");
}
