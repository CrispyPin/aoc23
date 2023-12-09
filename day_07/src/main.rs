use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

fn main() {
	let data = fs::read_to_string("input.txt").unwrap();
	let mut hands: Vec<_> = data
		.lines()
		.map(|line| line.split_once(' ').unwrap())
		.map(|(hand, multiplier)| {
			(
				array(hand),
				HandType::from(hand),
				multiplier.parse::<usize>().unwrap(),
				hand,
			)
		})
		.collect();
	hands.sort_by(|(hand_a, type_a, _, _), (hand_b, type_b, _, _)| {
		type_a.cmp(type_b).then(hand_a.cmp(hand_b))
	});
	// dbg!(&hands);

	let s: usize = hands
		.iter()
		.enumerate()
		// .inspect(|a| {
		// 	println!("{a:?}");
		// })
		.map(|(rank, (_, _, multiplier, _))| (rank + 1) * multiplier)
		.sum();
	println!("{s}");

	let mut hands: Vec<_> = data
		.lines()
		.map(|line| line.split_once(' ').unwrap())
		.map(|(hand, multiplier)| {
			(
				array_2(hand),
				HandType::jokered(hand),
				multiplier.parse::<usize>().unwrap(),
				hand,
			)
		})
		.collect();
	hands.sort_by(|(hand_a, type_a, _, _), (hand_b, type_b, _, _)| {
		type_a.cmp(type_b).then(hand_a.cmp(hand_b))
	});

	let s: usize = hands
		.iter()
		.enumerate()
		.map(|(rank, (_, _, multiplier, _))| (rank + 1) * multiplier)
		.sum();
	println!("{s}");
}

fn array(hand: &str) -> Vec<usize> {
	let values = "23456789TJQKA";
	hand.chars().map(|c| values.find(c).unwrap()).collect()
}

fn array_2(hand: &str) -> Vec<usize> {
	let values = "J23456789TQKA";
	hand.chars().map(|c| values.find(c).unwrap()).collect()
}

impl HandType {
	fn jokered(text: &str) -> Self {
		fn f(text: &str) -> Vec<HandType> {
			if text.contains('J') {
				"23456789TQKA"
					.chars()
					.map(|c| {
						//
						let s = String::from(c);
						f(&text.replace('J', &s))
					})
					.flatten()
					.collect()
			} else {
				vec![HandType::from(text)]
			}
		}

		let mut variants = f(text);
		variants.sort();
		*variants.last().unwrap()
	}

	fn from(text: &str) -> Self {
		let mut types = Vec::new();
		for c in text.chars() {
			if !types.contains(&c) {
				types.push(c);
			}
		}
		let amounts: Vec<_> = types.iter().map(|&t| text.matches(t).count()).collect();

		match types.len() {
			1 => Self::FiveOfAKind,
			2 => {
				if amounts.contains(&2) {
					Self::FullHouse
				} else {
					Self::FourOfAKind
				}
			}
			3 => {
				if amounts.contains(&3) {
					Self::ThreeOfAKind
				} else {
					Self::TwoPair
				}
			}
			4 => Self::OnePair,
			5 => Self::HighCard,
			_ => unreachable!(),
		}
	}
}
