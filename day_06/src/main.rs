use std::fs;

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

	println!("{} {}", big_time, big_record);
	println!("{}", beat(big_time, big_record));
}

fn beat(time: i64, record: i64) -> i64 {
	let mut variants = 0;
	for t in 0..time {
		let dist = t * (time - t);
		if dist > record {
			variants += 1;
		}
	}
	variants
}
