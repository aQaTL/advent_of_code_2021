use anyhow::anyhow;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
	let input = std::fs::read_to_string("day_16/input.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> anyhow::Result<u32> {
	let input: String = input
		.trim()
		.chars()
		.map(|c| format!("{:04b}", c.to_digit(16).unwrap() as u8))
		.flat_map(|str| str.chars().collect_vec())
		.collect();
	let packet_version_sum = parse_packet(&input, &mut 0)?;
	Ok(packet_version_sum)
}

fn parse_packet(input: &str, bit_idx: &mut usize) -> anyhow::Result<u32> {
	let mut packet_version_sum = 0;

	let packet_version = get_number(input, *bit_idx, 3)?;
	*bit_idx += 3;
	packet_version_sum += packet_version;

	let packet_type_id = get_number(input, *bit_idx, 3)?;
	*bit_idx += 3;

	match packet_type_id {
		4 => loop {
			*bit_idx += 5;
			if input.as_bytes()[*bit_idx - 5] == b'0' {
				break;
			}
		},
		_ => {
			let packet_length_type_id = get_number(input, *bit_idx, 1)?;
			*bit_idx += 1;
			if packet_length_type_id == 0 {
				let total_length_in_bits = get_number(input, *bit_idx, 15)? as usize;
				*bit_idx += 15;

				let final_bit_idx = *bit_idx + total_length_in_bits;
				while *bit_idx < final_bit_idx {
					packet_version_sum += parse_packet(input, bit_idx)?;
				}
			} else {
				let sub_packets_count = get_number(input, *bit_idx, 11)?;
				*bit_idx += 11;

				for _ in 0..sub_packets_count {
					packet_version_sum += parse_packet(input, bit_idx)?;
				}
			}
		}
	}
	Ok(packet_version_sum)
}

fn part_2(input: &str) -> anyhow::Result<i64> {
	let input: String = input
		.trim()
		.chars()
		.map(|c| format!("{:04b}", c.to_digit(16).unwrap() as u8))
		.flat_map(|str| str.chars().collect_vec())
		.collect();
	let expr_result = parse_packet_p2(&input, &mut 0)?;
	Ok(expr_result)
}

fn parse_packet_p2(input: &str, bit_idx: &mut usize) -> anyhow::Result<i64> {
	let _packet_version = get_number(input, *bit_idx, 3)?;
	*bit_idx += 3;

	let packet_type_id = get_number(input, *bit_idx, 3)?;
	*bit_idx += 3;

	let mut packets = Vec::new();
	match packet_type_id {
		4 => {
			let mut bytes = String::new();
			loop {
				bytes.push_str(&input[(*bit_idx + 1)..(*bit_idx + 5)]);
				*bit_idx += 5;
				if input.as_bytes()[*bit_idx - 5] == b'0' {
					break;
				}
			}
			let n = u64::from_str_radix(&bytes, 2)?;
			return Ok(n as i64);
		}
		_ => {
			let packet_length_type_id = get_number(input, *bit_idx, 1)?;
			*bit_idx += 1;
			if packet_length_type_id == 0 {
				let total_length_in_bits = get_number(input, *bit_idx, 15)? as usize;
				*bit_idx += 15;

				let final_bit_idx = *bit_idx + total_length_in_bits;
				while *bit_idx < final_bit_idx {
					packets.push(parse_packet_p2(input, bit_idx)?);
				}
			} else {
				let sub_packets_count = get_number(input, *bit_idx, 11)?;
				*bit_idx += 11;

				for _ in 0..sub_packets_count {
					packets.push(parse_packet_p2(input, bit_idx)?);
				}
			}
		}
	}

	match packet_type_id {
		0 => Ok(packets.iter().sum()),
		1 => Ok(packets.iter().product()),
		2 => Ok(*packets
			.iter()
			.min()
			.ok_or_else(|| anyhow!("Failed to find min value"))?),
		3 => Ok(*packets
			.iter()
			.max()
			.ok_or_else(|| anyhow!("Failed to find max value"))?),
		5 => Ok(if packets[0] > packets[1] { 1 } else { 0 }),
		6 => Ok(if packets[0] < packets[1] { 1 } else { 0 }),
		7 => Ok(if packets[0] == packets[1] { 1 } else { 0 }),
		_ => Err(anyhow!("Unexpected packet type id: {}", packet_type_id)),
	}
}

fn get_number(input: &str, bit_idx: usize, len: usize) -> anyhow::Result<u32> {
	u32::from_str_radix(&input[bit_idx..bit_idx + len], 2).map_err(Into::into)
}

#[cfg(test)]
mod tests {
	static EX_1: &str = "D2FE28";
	// static EX_2: &str = "38006F45291200";
	// static EX_3: &str = "EE00D40C823060";
	static EX_4: &str = "8A004A801A8002F478";
	static EX_5: &str = "620080001611562C8802118E34";
	static EX_6: &str = "C0015000016115A2E0802F182340";
	static EX_7: &str = "A0016C880162017C3686B18A3D4780";

	#[test]
	fn part_1_ex_1() {
		assert_eq!(super::part_1(EX_1).unwrap(), 6);
	}

	#[test]
	fn part_1_ex_4() {
		assert_eq!(super::part_1(EX_4).unwrap(), 16);
	}

	#[test]
	fn part_1_ex_5() {
		assert_eq!(super::part_1(EX_5).unwrap(), 12);
	}

	#[test]
	fn part_1_ex_6() {
		assert_eq!(super::part_1(EX_6).unwrap(), 23);
	}

	#[test]
	fn part_1_ex_7() {
		assert_eq!(super::part_1(EX_7).unwrap(), 31);
	}

	static EX_8: &str = "C200B40A82";
	static EX_9: &str = "04005AC33890";
	static EX_10: &str = "880086C3E88112";
	static EX_11: &str = "CE00C43D881120";
	static EX_12: &str = "D8005AC2A8F0";
	static EX_13: &str = "F600BC2D8F";
	static EX_14: &str = "9C005AC2F8F0";
	static EX_15: &str = "9C0141080250320F1802104A08";

	#[test]
	fn part_2_ex_8() {
		assert_eq!(super::part_2(EX_8).unwrap(), 3);
	}

	#[test]
	fn part_2_ex_9() {
		assert_eq!(super::part_2(EX_9).unwrap(), 54);
	}

	#[test]
	fn part_2_ex_10() {
		assert_eq!(super::part_2(EX_10).unwrap(), 7);
	}

	#[test]
	fn part_2_ex_11() {
		assert_eq!(super::part_2(EX_11).unwrap(), 9);
	}

	#[test]
	fn part_2_ex_12() {
		assert_eq!(super::part_2(EX_12).unwrap(), 1);
	}

	#[test]
	fn part_2_ex_13() {
		assert_eq!(super::part_2(EX_13).unwrap(), 0);
	}

	#[test]
	fn part_2_ex_14() {
		assert_eq!(super::part_2(EX_14).unwrap(), 0);
	}

	#[test]
	fn part_2_ex_15() {
		assert_eq!(super::part_2(EX_15).unwrap(), 1);
	}
}
