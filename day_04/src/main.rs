use std::fs;

fn main () {
	for _ in 0..1000{
	a();
	b();}
}

fn a() {
	let bytes = fs::read("input.txt").expect("input.txt not found");
	let mut total_score = 0u16;
	let mut p = 0;
	while p < bytes.len() {
		p += 10;
		let mut winners = [0; 10];
		for w in 0..10 {
			if bytes[p] != b' ' {
				winners[w] = (bytes[p] - 48) * 10;
			}
			winners[w] += bytes[p + 1] - 48;
			p += 3;
		}
		p += 2;
		let mut score: u16 = 0;
		for _ in 0..25 {
			let mut n = bytes[p + 1] - 48;
			if bytes[p] != b' ' {
				n += (bytes[p] - 48) * 10;
			}
			p += 3;
			if winners.contains(&n) {
				if score == 0 {
					score = 1;
				} else {
					score = score << 1;
				}
			}
		}
		total_score += score;
	}
	println!("{total_score}");
}

fn b() {
	let bytes = fs::read("input.txt").expect("input.txt not found");
	let mut cards = vec![1];
	let mut p = 0;
	let mut card_id = 0;
	while p < bytes.len() {
		p += 10;
		let mut winners = [0; 10];
		for w in 0..10 {
			if bytes[p] != b' ' {
				winners[w] = (bytes[p] - 48) * 10;
			}
			winners[w] += bytes[p + 1] - 48;
			p += 3;
		}
		p += 2;
		let mut score = 0;
		for _ in 0..25 {
			let mut n = bytes[p + 1] - 48;
			if bytes[p] != b' ' {
				n += (bytes[p] - 48) * 10;
			}
			p += 3;
			if winners.contains(&n) {
				score += 1;
			}
		}
		if card_id == cards.len() {
			cards.push(1);
		}
		let this_card = cards[card_id];
		for i in 1..(score+1) {
			if card_id + i >= cards.len() {
				cards.push(1 + this_card);
			} else {
				cards[card_id + i] += this_card;
			}
		}
		card_id += 1;
	}
	let sum: usize = cards.iter().sum();
	println!("{}", sum);
}
