struct Game {
	id: u32,
	red: u32,
	green: u32,
	blue: u32,
}

fn main() {
	let data = include_str!("../2.txt");
	let sum: u32 = data
		.lines()
		.map(|line| {
			let (title, contents) = line.split_once(": ").unwrap();
			let id = title.split_once(' ').unwrap().1.parse().unwrap();
			let mut game = Game {
				id,
				red: 0,
				green: 0,
				blue: 0,
			};
			for hand in contents.split("; ") {
				for t in hand.split(", ") {
					let (count, col) = t.split_once(" ").unwrap();
					let count = count.parse().unwrap();
					match col {
						"red" => game.red = game.red.max(count),
						"green" => game.green = game.green.max(count),
						"blue" => game.blue = game.blue.max(count),
						_ => panic!(),
					}
				}
			}
			game
		})
		.filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
		.map(|game| game.id)
		.sum();

	println!("{sum}");
	let sum: u32 = data
		.lines()
		.map(|line| {
			let (title, contents) = line.split_once(": ").unwrap();
			let id = title.split_once(' ').unwrap().1.parse().unwrap();
			let mut game = Game {
				id,
				red: 0,
				green: 0,
				blue: 0,
			};
			for hand in contents.split("; ") {
				for t in hand.split(", ") {
					let (count, col) = t.split_once(" ").unwrap();
					let count = count.parse().unwrap();
					match col {
						"red" => game.red = game.red.max(count),
						"green" => game.green = game.green.max(count),
						"blue" => game.blue = game.blue.max(count),
						_ => panic!(),
					}
				}
			}
			game
		})
		.map(|game| game.red * game.blue * game.green)
		.sum();
	println!("{sum}");
}
