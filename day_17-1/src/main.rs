use std::collections::HashMap;

struct Range {
	min: i32,
	max: i32,
}

struct Area {
	x: Range,
	y: Range,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
	x: i32,
	y: i32,
}

fn main() {
	let target_area = Area {
		x: Range {
			min: 195,
			max: 238,
		},
		y: Range {
			min: -93,
			max: -67,
		}
	};

	let steps = shoot(&target_area, Point {x: 20, y: 92});
	println!("steps: {:?}", steps);
	println!("Max height: {}", max_y(&steps));
	// print_map(&target_area, &steps);
}

fn shoot(target_area: &Area, mut curr_velocity: Point) -> Vec<Point> {
	let mut steps = Vec::new();
	let mut curr_point = Point {
		x: 0,
		y: 0,
	};
	
	loop {
		curr_point = next_step(&curr_velocity, &curr_point);
		curr_velocity = update_velocity(curr_velocity);
		if !valid_shot(target_area, &curr_point) {
			break;
		}
		steps.push(curr_point);
	}
	steps
}

fn valid_shot(target_area: &Area, curr_point: &Point) -> bool {
	if curr_point.x > target_area.x.max
		|| curr_point.y < target_area.y.min {
		false
	} else {
		true
	}
}

fn next_step(velocity: &Point, curr_point: &Point) -> Point {
	Point {
		x: curr_point.x + velocity.x,
		y: curr_point.y + velocity.y,
	}
}

fn update_velocity(mut new_velocity: Point) -> Point {
	if new_velocity.x > 0 {
		new_velocity.x -= 1;
	} else if new_velocity.x < 0 {
		new_velocity.x += 1;
	}
	new_velocity.y -= 1;
	new_velocity
}

fn print_map(target_area: &Area, steps: &[Point]) {
	let mut map: HashMap<Point, char> = HashMap::new();
	let max_y = max_y(steps);

	for y in target_area.y.min..max_y + 1 {
		for x in 0..target_area.x.max + 1 {
			map.insert(Point {x, y}, '.');
		}
	}
	map.insert(Point {x: 0, y: 0}, 'S');
	for y in target_area.y.min..target_area.y.max + 1 {
		for x in target_area.x.min..target_area.x.max + 1 {
			map.insert(Point {x, y}, 'T');
		}
	}
	for step in steps {
		map.insert(*step, '#');
	}
	
	for y in (target_area.y.min..max_y + 1).rev() {
		for x in 0..target_area.x.max + 1 {
			print!("{}", map.get(&Point {x, y}).unwrap());
		}
		println!("");
	}
}

fn max_y(steps: &[Point]) -> i32 {
	let mut max = 0;
	let mut last_y = 0;

	for step in steps {
		if step.y > max {
			max = step.y;
		} else if last_y != 0 && step.y < last_y {
			break;
		}
		last_y = step.y;
	}
	max
}
