use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const VERSION_SIZE: usize = 3;
const TYPE_ID_SIZE: usize = 3;
const HEADER_SIZE: usize = 6;
const LENGTH_TYPE_SIZE: usize = 1;
const LENGTH_VALUE_ONE: usize = 15;
const LENGTH_VALUE_TWO: usize = 11;

const NOT_SET: usize = 0;

#[derive(Debug, PartialEq)]
struct Packet {
	version: usize,
	type_id: TypeID,
	len: usize,
	info: PacketType,
}

#[derive(Debug, PartialEq, Eq)]
enum TypeID {
	Sum,
	Product,
	Minimum,
	Maximum,
	SingleNumber,
	GreaterThan,
	LessThan,
	EqualTo,
}

#[derive(Debug, PartialEq)]
enum PacketType {
	Literal(LiteralValue),
	Operator(OperatorPacket),
}

#[derive(Debug, PartialEq)]
struct LiteralValue {
	number: u128,
}

#[derive(Debug, PartialEq)]
struct OperatorPacket {
	length_type: char,
	length_value: usize,
	sub_packets: Vec<Packet>,
}

impl OperatorPacket {
	fn new() -> OperatorPacket {
		OperatorPacket {
			length_type: '1',
			length_value: 0,
			sub_packets: Vec::new(),
		}
	}
}

trait RemoveFromStart {
	fn remove_from_start(&mut self, chars_to_remove: usize) -> &mut str;
}

impl RemoveFromStart for str {
	fn remove_from_start(&mut self, chars_to_remove: usize) -> &mut str {
		self.get_mut(chars_to_remove..).unwrap()
	}
}

fn main() {
	let input_lines = read_lines("input")
		.expect("Can't read the input file");

	let mut input_bin = init_binary(&input_lines.flatten().next().unwrap(), hex_to_bin_table());
	let packets: Vec<Packet> = vec![parse_binaries(&mut input_bin)];

	println!("packets: {:#?}", packets);

	println!("Versions sum: {}", versions_sum(packets.first().unwrap()));
	println!("Calculated result: {}", calculate(packets.first().unwrap()));
}

fn hex_to_bin_table() -> HashMap<char, String> {
	let mut hex_to_bin: HashMap<char, String> = HashMap::new();

	hex_to_bin.insert('0', "0000".to_string());
	hex_to_bin.insert('1', "0001".to_string());
	hex_to_bin.insert('2', "0010".to_string());
	hex_to_bin.insert('3', "0011".to_string());
	hex_to_bin.insert('4', "0100".to_string());
	hex_to_bin.insert('5', "0101".to_string());
	hex_to_bin.insert('6', "0110".to_string());
	hex_to_bin.insert('7', "0111".to_string());
	hex_to_bin.insert('8', "1000".to_string());
	hex_to_bin.insert('9', "1001".to_string());
	hex_to_bin.insert('A', "1010".to_string());
	hex_to_bin.insert('B', "1011".to_string());
	hex_to_bin.insert('C', "1100".to_string());
	hex_to_bin.insert('D', "1101".to_string());
	hex_to_bin.insert('E', "1110".to_string());
	hex_to_bin.insert('F', "1111".to_string());
	hex_to_bin
}

fn init_binary(s: &str, hex_to_bin_table: HashMap<char, String>)
	-> String {
	let mut input_bin: String = String::new();

	println!("Original:\n{}", s);
	for c in s.chars() {
		input_bin.push_str(hex_to_bin_table.get(&c).unwrap());
	}
	println!("Binary:\n{}", input_bin);
	input_bin
}

fn parse_binaries(mut binaries: &mut str) -> Packet {
	let version = bin_to_usize(&binaries[..VERSION_SIZE]);
	binaries = binaries.remove_from_start(VERSION_SIZE);

	let type_id = usize_to_type_id(bin_to_usize(&binaries[..TYPE_ID_SIZE]));
	binaries = binaries.remove_from_start(TYPE_ID_SIZE);

	if type_id == TypeID::SingleNumber {
		save_literal(&mut binaries, version, type_id, NOT_SET)
	} else {
		save_operator(&mut binaries, version, type_id)
	}
}

fn save_literal(binaries: &mut str, version: usize, type_id: TypeID, packet_size: usize)
	-> Packet {
	let nb_chars;
	
	if packet_size == NOT_SET {
		nb_chars = set_dynamic_size_nb(binaries);
	} else {
		nb_chars = binaries[0..packet_size - HEADER_SIZE].to_string();
	}

	println!("Literal in binary: {}", nb_chars);
	Packet {
		version,
		type_id,
		len: VERSION_SIZE + TYPE_ID_SIZE + nb_chars.len() + nb_chars.len() / 4,
		info: PacketType::Literal(LiteralValue {
			number: bin_to_u128(&nb_chars)}),
	}
}

fn set_dynamic_size_nb(binaries: &str) -> String {
	let mut nb_chars = String::new();
	let mut continue_to_read = true;
	
	for (pos, c) in binaries.chars().enumerate() {
		if pos % 5 == 0 {
			if !continue_to_read {
				break;
			}
			else if c == '0' {
				continue_to_read = false;
			}
		}
		else {
			nb_chars.push(c);
		}
	}
	nb_chars
}

fn save_operator(mut binaries: &mut str, version: usize, type_id: TypeID)
	-> Packet {
	let mut operator_packet: OperatorPacket = OperatorPacket::new();
	let mut operator_len = VERSION_SIZE + TYPE_ID_SIZE + LENGTH_TYPE_SIZE;
	let length_bites;

	operator_packet.length_type = binaries.chars().next().unwrap();
	binaries = binaries.remove_from_start(1);
	
	if operator_packet.length_type == '0' {
		length_bites = LENGTH_VALUE_ONE;
	} else {
		length_bites = LENGTH_VALUE_TWO;
	}
	operator_len += length_bites;
	operator_packet.length_value = bin_to_usize(&binaries[..length_bites]);
	binaries = binaries.remove_from_start(length_bites);

	if length_bites == LENGTH_VALUE_ONE {
		operator_len += operator_packet.length_value;

		let mut consumed_bites = 0;
		while consumed_bites < operator_packet.length_value {
			operator_packet.sub_packets.push(
				parse_binaries(&mut binaries[..operator_packet.length_value - consumed_bites]));
			let last_sub_packet_len = operator_packet.sub_packets.last().unwrap().len;
			consumed_bites += last_sub_packet_len;
			binaries = binaries.remove_from_start(last_sub_packet_len);
		}
	}
	else {
		for _i in 0..operator_packet.length_value {
			operator_packet.sub_packets.push(parse_binaries(&mut binaries));
			let last_sub_packet_len = operator_packet.sub_packets.last().unwrap().len;
			operator_len += last_sub_packet_len;
			binaries = binaries.remove_from_start(last_sub_packet_len);
			println!("last operator len: {}", last_sub_packet_len);
		}
	}
	
	Packet {
		version,
		type_id,
		len: operator_len,
		info: PacketType::Operator(operator_packet)
	}
}

fn bin_to_usize(s: &str) -> usize {
	let mut int_nb: usize = 0;
	let base: i32 = 2;

	for (pos, c) in s.chars().enumerate() {
		if c == '1' {
			int_nb += base.pow((s.len() - pos - 1) as u32) as usize;
		}
	}
	int_nb
}

fn bin_to_u128(s: &str) -> u128 {
	let mut int_nb: u128 = 0;
	let base: u128 = 2;

	for (pos, c) in s.chars().enumerate() {
		if c == '1' {
			int_nb += base.pow((s.len() - pos - 1) as u32) as u128;
		}
	}
	int_nb
}

fn usize_to_type_id(nb: usize) -> TypeID {
	match nb {
		0 => TypeID::Sum,
		1 => TypeID::Product,
		2 => TypeID::Minimum,
		3 => TypeID::Maximum,
		4 => TypeID::SingleNumber,
		5 => TypeID::GreaterThan,
		6 => TypeID::LessThan,
		_other => TypeID::EqualTo,
	}
}

fn versions_sum(packet: &Packet) -> usize {
	let mut sum: usize = packet.version;

	match &packet.info {
		PacketType::Operator(operator) => {
			for sub_packet in &operator.sub_packets {
				sum += versions_sum(sub_packet);
			}
		},
		PacketType::Literal(_) => (),
	}
	sum
}

fn calculate(packet: &Packet) -> u128 {
	match &packet.info {
		PacketType::Operator(operator) => {
			match &packet.type_id {
				TypeID::Sum => {
					op_sum(&operator.sub_packets)
				},
				TypeID::Product => {
					op_product(&operator.sub_packets)
				},
				TypeID::Minimum => {
					op_minimum(&operator.sub_packets)
				},
				TypeID::Maximum => {
					op_maximum(&operator.sub_packets)
				},
				TypeID::GreaterThan => {
					op_greater_than(&operator.sub_packets)
				},
				TypeID::LessThan => {
					op_less_than(&operator.sub_packets)
				},
				TypeID::EqualTo => {
					op_equal_to(&operator.sub_packets)
				},
				_ => {
					panic!("Unexpected typeID for an Operator")
				}
			}
		},
		PacketType::Literal(literal) => literal.number,
	}
}

fn op_sum(packets: &[Packet]) -> u128 {
	let mut sum = 0;

	for packet in packets {
		match &packet.info {
			PacketType::Literal(info) => sum += info.number,
			PacketType::Operator(_) => sum += calculate(packet),
		}
	}
	sum
}

fn op_product(packets: &[Packet]) -> u128 {
	let mut product = 1;

	for packet in packets {
		match &packet.info {
			PacketType::Literal(info) => product *= info.number,
			PacketType::Operator(_) => product *= calculate(packet),
		}
	}
	product
}

fn op_minimum(packets: &[Packet]) -> u128 {
	let mut numbers = Vec::new();

	for packet in packets {
		match &packet.info {
			PacketType::Literal(info) => numbers.push(info.number),
			PacketType::Operator(_) => numbers.push(calculate(packet)),
		}
	}
	numbers.sort_unstable();
	*numbers.first().unwrap()
}

fn op_maximum(packets: &[Packet]) -> u128 {
	let mut numbers = Vec::new();

	for packet in packets {
		match &packet.info {
			PacketType::Literal(info) => numbers.push(info.number),
			PacketType::Operator(_) => numbers.push(calculate(packet)),
		}
	}
	numbers.sort_unstable();
	numbers.reverse();
	*numbers.first().unwrap()
}

fn op_greater_than(packets: &[Packet]) -> u128 {
	let (nb1, nb2) = take_two_numbers(packets);
	if nb1 > nb2 {
		1
	}
	else {
		0
	}
}

fn op_less_than(packets: &[Packet]) -> u128 {
	let (nb1, nb2) = take_two_numbers(packets);
	if nb1 < nb2 {
		1
	}
	else {
		0
	}
}

fn op_equal_to(packets: &[Packet]) -> u128 {
	let (nb1, nb2) = take_two_numbers(packets);
	if nb1 == nb2 {
		1
	}
	else {
		0
	}
}

fn take_two_numbers(packets: &[Packet]) -> (u128, u128) {
	let nb1;
	let nb2;

	if packets.len() != 2 {
		panic!("Comparison operators must have exaclty 2 parameters");
	}
	let packet1 = &packets[0];
	match &packet1.info {
		PacketType::Literal(info) => nb1 = info.number,
		PacketType::Operator(_) => nb1 = calculate(packet1),
	}
	let packet2 = &packets[1];
	match &packet2.info {
		PacketType::Literal(info) => nb2 = info.number,
		PacketType::Operator(_) => nb2 = calculate(packet2),
	}
	(nb1, nb2)
}

// Function taken from the Rust manual
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
