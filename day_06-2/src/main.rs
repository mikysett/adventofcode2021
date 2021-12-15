use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct FishDay {
	size: u64,
	deliverers: u64,
}

const DAYS: u32 = 256;

fn main() {
	let lanternfish: Vec<u32> = fs::read_to_string("input").unwrap()
		.split(',')
		.map(|s| s.trim())
		.map(|s| s.parse().unwrap())
		.collect();
	let mut fish_calendar = set_calendar();

	let mut total = 0;
	for fish in lanternfish {
		let sgl_fish = fish_calendar.entry(DAYS - fish)
			.or_insert(FishDay{ size: 0, deliverers: 0 });
		total += sgl_fish.size;
	}
	println!("Total (FAST calc): {}", total);
}

fn set_calendar() -> HashMap<u32, FishDay> {
	let mut last_size = 1;
	let mut fish_calendar = HashMap::new();
	fish_calendar.entry(1).or_insert(FishDay {
		size: 1,
		deliverers: 1,
	});
	for day in 1..DAYS + 1 {
		let curr_day = fish_calendar.entry(day).or_insert(FishDay {
			size: last_size,
			deliverers: 0,
		});
		if curr_day.deliverers != 0 {
			last_size += curr_day.deliverers;
			curr_day.size = last_size;
			let newborn = curr_day.deliverers;
			if day + 7 <= DAYS {
				let deliver_day = fish_calendar.entry(day + 7).or_insert(FishDay {
					size: 0,
					deliverers: 0,
				});
				deliver_day.deliverers += newborn;
			}
			if day + 9 <= DAYS {
				let deliver_day = fish_calendar.entry(day + 9).or_insert(FishDay {
					size: 0,
					deliverers: 0,
				});
				deliver_day.deliverers += newborn;
			}
		}
	}
	fish_calendar
}