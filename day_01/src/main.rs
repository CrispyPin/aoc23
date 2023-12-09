use std::fs;

fn main() {
	// for _ in 0..1000 {
	// 	a();
	// }
	a();
	b();
}

fn a() {
	/*
	let data = include_str!("../1.txt");
	let sum: i32 = data
		.lines()
		.map(|line| {
			let nums: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
			let mut num = nums[0].to_string();
			num.push(*nums.last().unwrap());
			num.parse::<i32>().unwrap()
		})
		.sum();
	println!("{sum:?}");
	*/

	let bytes = fs::read("input.txt").expect("input.txt not found");
	let mut sum = 0;
	let mut first_digit = 0;
	let mut last_digit = 0;
	for byte in bytes {
		if byte <= b'9' && byte > b'0' {
			let digit = byte - 48;
			last_digit = digit;
			if first_digit == 0 {
				first_digit = digit * 10;
			}
		} else if byte == b'\n' {
			sum += last_digit as u16;
			sum += first_digit as u16;
			last_digit = 0;
			first_digit = 0;
		}
	}
	println!("{sum}");
}

fn b() {
	let data = include_str!("../1.txt");

	let sum: i32 = data
		.lines()
		.map(|line| {
			let mut digits = Vec::new();
			for i in 0..line.len() {
				let c = line.chars().nth(i).unwrap();
				if c.is_numeric() {
					digits.push(c);
				} else {
					for (digit, &text) in [
						"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
					]
					.iter()
					.enumerate()
					.map(|(i, t)| ((i + 1).to_string().chars().next().unwrap(), t))
					{
						if line[i..].starts_with(text) {
							digits.push(digit);
							break;
						}
					}
				}
			}

			let mut num = digits[0].to_string();
			num.push(*digits.last().unwrap());
			num.parse::<i32>().unwrap()
		})
		.sum();
	println!("{sum:?}");
}
