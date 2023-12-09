use std::fs;

fn main() {
	let data = fs::read_to_string("input.txt").expect("input.txt not found");
	let (seed_text, maps) = data.split_once("\n\n").unwrap();
	let seeds: Vec<usize> = seed_text[7..]
		.split_whitespace()
		.map(|s| s.parse().unwrap())
		.collect();

	let maps: Vec<Vec<(usize, usize, usize)>> = maps
		.split("\n\n")
		.map(|mapdata| {
			let (_, mapdata) = mapdata.split_once('\n').unwrap();
			mapdata
				.lines()
				.map(|line| {
					let (dest_start, tmp) = line.split_once(' ').unwrap();
					let (source_start, range_len) = tmp.split_once(' ').unwrap();
					(
						dest_start.parse().unwrap(),
						source_start.parse().unwrap(),
						range_len.parse().unwrap(),
					)
				})
				.collect()
		})
		.collect();
	let mut min_last_map = usize::MAX;
	for &seed in &seeds {
		let mut value = seed;
		for map in &maps {
			for &(dest_start, source_start, range_len) in map {
				if value >= source_start && value < source_start + range_len {
					value = dest_start + (value - source_start);
					break;
				}
			}
		}
		min_last_map = min_last_map.min(value);
	}
	println!("{min_last_map}");

	let seed_ranges: Vec<_> = (0..(seeds.len() / 2))
		.map(|i| seeds[i]..seeds[i + 1])
		.collect();
	let mut min_last_map = usize::MAX;
	for seed_range in &seed_ranges {
		for seed in seed_range.clone() {
			let mut value = seed;
			for map in &maps {
				for &(dest_start, source_start, range_len) in map {
					if value >= source_start && value < source_start + range_len {
						value = dest_start + (value - source_start);
						break;
					}
				}
			}
			min_last_map = min_last_map.min(value);
		}
	}
	println!("{min_last_map}");
}
