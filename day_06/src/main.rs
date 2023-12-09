use std::fs;

use num_integer::Roots;

fn main() {
	let data = fs::read_to_string("input.txt").unwrap();
	let (time_str, record_str) = data.split_once('\n').unwrap();
	let times: Vec<i64> = time_str
		.split_whitespace()
		.skip(1)
		.map(|t| t.parse().unwrap())
		.collect();
	let records: Vec<i64> = record_str
		.split_whitespace()
		.skip(1)
		.map(|t| t.parse().unwrap())
		.collect();
	let answer = times
		.iter()
		.zip(records.iter())
		.map(|(&time, &record)| beat(time, record))
		.fold(1, |a, b| a * b);
	println!("{answer}");

	let big_time: i64 = time_str[7..].replace(' ', "").parse().unwrap();
	let big_record: i64 = record_str[9..].trim().replace(' ', "").parse().unwrap();

	/*
	distance = held * (time - held)

	held * (time - held) = record
	held*time - held*held = record
	held*time - held*held - record = 0
	held*held - held*time + record = 0
	*/
	// let lower = ((time * time / 4) - record).sqrt() - time / 2;
	// let upper = -((time * time / 4) - record).sqrt() - time / 2;
	// println!("{lower} -> {upper}");
}

fn beat(time: i64, record: i64) -> i64 {
	let mut variants = 0;
	for t in 0..time {
		let dist = t * (time - t);
		if dist > record {
			variants += 1;
		}
	}

	let time = time as f64;
	let record = record as f64;
	let lower = ((time * time / 4.0) - record).sqrt() - time / 2.0;
	let upper = -((time * time / 4.0) - record).sqrt() - time / 2.0;
	println!("{lower} -> {upper}");

	variants
}
