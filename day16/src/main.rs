use std::fs;
use bit_vec::BitVec;
use std::u8;

use aoc_utils::{AoCSolution, run};

struct Solution {}

fn decode_hex_string(s: &str) -> BitVec {
  let bits = s.chars().fold(BitVec::new(),|mut acc,c| {
    let val = u8::from_str_radix(&c.to_string(), 16).unwrap();
    acc.push(val & 0b1000 > 0);
    acc.push(val & 0b0100 > 0);
    acc.push(val & 0b0010 > 0);
    acc.push(val & 0b0001 > 0);
    return acc;
  });
  return bits;
}

fn bits_to_decimal(b: &BitVec) -> u32 {
  return b.iter().rev().enumerate().fold(0,|acc, (i,b)| acc + if b { (2 as u32).pow(i as u32) } else {0} )
}

#[derive(Debug)]
struct Packet<'a> {
  bits: &'a BitVec,
  start: usize
}

fn print_packet(p: &Packet) {
  println!("type: {}, version: {}, ltype: {}", p.get_type(), p.get_version(), p.get_length_type());
  println!("data: {:?}", p.get_bits());
}

fn get_literal(bits: &BitVec) -> (u32,usize) {
  let mut value = BitVec::new();
    for i in (0..bits.len()-5).step_by(5) {
      value.push(bits[i+1]);
      value.push(bits[i+2]);
      value.push(bits[i+3]);
      value.push(bits[i+4]);
      if !bits[i] {
        return (bits_to_decimal(&value),i+5);
      }
    }
  return (0,0);
}

impl<'a> Packet<'a> {

  fn get_bits(&self) -> BitVec {
    return self.bits.iter().skip(self.start).collect();
  }

  fn get_header_bits(&self) -> BitVec {
    if self.get_type() == 4 {
      return self.get_bits().iter().take(6).collect();
    } else if self.get_length_type() == 0 {
      return self.get_bits().iter().take(7+15).collect()
    } else {
      return self.get_bits().iter().take(7+11).collect()
    }
  }

  fn get_data_bits(&self) -> BitVec {
    if self.get_type() == 4 {
      return self.get_bits().iter().skip(6).collect();
    } else if self.get_length_type() == 0 {
      return self.get_bits().iter().skip(7+15).collect()
    } else {
      return self.get_bits().iter().skip(7+11).collect()
    }
  }

  fn get_version(&self) -> u8 {
    let bytes = self.get_bits().to_bytes();
    return bytes[0] >> 5;
  }

  fn get_type(&self) -> u8 {
    let bytes = self.get_bits().to_bytes();
    return bytes[0] >> 2 & 0b111;
  }

  fn get_length_type(&self) -> u8 {
    return self.get_bits()[6] as u8;
  }

  fn get_subpacket_length(&self) -> usize {
    if self.get_type() == 4 {
      return 0;
    }
    let vec = &self.get_header_bits().iter().skip(7).collect();
    return bits_to_decimal(&vec) as usize
  }

  fn get_length(&self) -> usize {
    if self.get_type() == 4 {
      return self.get_header_bits().len() + get_literal(&self.get_data_bits()).1;
    } else if self.get_length_type() == 0 {
      let length_in_bits = self.get_subpacket_length();
      return self.get_header_bits().len() + length_in_bits as usize;
    } else {
      let length_in_packets = self.get_subpacket_length();
      let mut head = 0;
      for i in 0..length_in_packets {
        let packet = Packet{bits: self.bits, start: self.start + self.get_header_bits().len() + head};
        head += packet.get_length();
      }
      return self.get_header_bits().len() + head;
    }
  }

  fn get_subpackets(&self) -> Vec<Packet> {
    let mut subpackets = vec![];

    if self.get_type() == 4 {
      // do nothing
    } else if self.get_length_type() == 0 {
      let length_in_bits = self.get_subpacket_length();
      let mut head = 0;
      while head < length_in_bits - 6 {
        let packet = Packet{ bits: self.bits, start: self.start + self.get_header_bits().len() + head};
        head += packet.get_length();
        subpackets.push(packet);
      }
    } else {
      let length_in_packets = self.get_subpacket_length();
      let mut head = 0;
      for i in 0..length_in_packets {
        let packet = Packet{ bits: self.bits, start: self.start + self.get_header_bits().len() + head};
        head += packet.get_length();
        subpackets.push(packet);
      }
    }

    return subpackets;
  }

  fn get_literal(&self) -> Option<u32> {
    if self.get_type() != 4 {
      return None;
    }
    return Some(get_literal(&self.get_data_bits()).0);
  }
}

impl AoCSolution for Solution  {
    fn part1(&self, input: &String) -> i64 {
      let trimmed_input = input.trim();

      let packet = Packet { bits: &decode_hex_string(trimmed_input), start: 0 };

      fn sum_packet_versions(p: &Packet) -> u32 {
        if p.get_type() == 4 {
          return p.get_version() as u32;
        } else {
          return p.get_version() as u32 + p.get_subpackets().iter().fold(0, |acc, sp| {
            return acc + sum_packet_versions(sp) as u32;
          })
        }
      }

      return sum_packet_versions(&packet) as i64;
    }
    
    fn part2(&self, input: &String) -> i64 {

      return 0;
    }
}

fn main() {
    run(
        "Advent of Code day 16", 
        &fs::read_to_string("input.txt").expect("Input Error"), 
        &Solution {}
    );
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_decode_hex_string() {
    let decoded = decode_hex_string("D2FE28");
    assert_eq!(decoded, BitVec::from_bytes(&[0b11010010, 0b11111110, 0b00101000]));
  }

  #[test]
  fn test_decode_bit_sequence() {
    let packet = Packet { bits: &decode_hex_string("D2FE28"), start: 0 };
    assert_eq!(packet.get_version(), 6);
    assert_eq!(packet.get_type(), 4);
  }

  #[test]
  fn test_get_literal() {
    let packet1 = Packet { bits: &decode_hex_string("D2FE28"), start: 0 };
    assert_eq!(packet1.get_literal(), Some(2021));
    let packet2 = Packet { bits: &decode_hex_string("38006F45291200"), start: 0 };
    assert_eq!(packet2.get_literal(), None);
  }

  #[test]
  fn test_get_subpacket_length() {
    let packet1 = Packet { bits: &decode_hex_string("38006F45291200"), start: 0 };
    assert_eq!(packet1.get_subpacket_length(), 27);
    let packet2 = Packet { bits: &decode_hex_string("EE00D40C823060"), start: 0 };
    assert_eq!(packet2.get_subpacket_length(), 3);
  }

  #[test]
  fn test_get_length() {
    let packet1 = Packet { bits: &decode_hex_string("38006F45291200"), start: 0 };
    assert_eq!(packet1.get_length(), 27 + 22);
    let packet2 = Packet { bits: &decode_hex_string("EE00D40C823060"), start: 0 };
    assert_eq!(packet2.get_length(), 33 + 18);
  }

  #[test]
  fn test_get_subpackets1() {
    let packet = Packet { bits: &decode_hex_string("38006F45291200"), start: 0 };
    let subpackets = packet.get_subpackets();
    assert_eq!(subpackets[0].get_literal(), Some(10));
    assert_eq!(subpackets[1].get_literal(), Some(20));
  }

  #[test]
  fn test_get_subpackets2() {
    let packet = Packet { bits: &decode_hex_string("EE00D40C823060"), start: 0 };
    let subpackets = packet.get_subpackets();
    assert_eq!(subpackets[0].get_literal(), Some(1));
    assert_eq!(subpackets[1].get_literal(), Some(2));
    assert_eq!(subpackets[2].get_literal(), Some(3));
  }


  #[test]
  fn test_part1_1() {
    let input = String::from(indoc!{"
    8A004A801A8002F478
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 16);
  }

  #[test]
  fn test_part1_2() {
    let input = String::from(indoc!{"
    620080001611562C8802118E34
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 12);
  }

  #[test]
  fn test_part1_3() {
    let input = String::from(indoc!{"
    C0015000016115A2E0802F182340
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 23);
  }

  #[test]
  fn test_part1_4() {
    let input = String::from(indoc!{"
    A0016C880162017C3686B18A3D4780
    "});
    let result = (&Solution {}).part1(&input);
    assert_eq!(result, 31);
  }

  #[test]
  fn test_part2_1() {
    let input = String::from(indoc!{"
    D2FE28
    "});
    let result = (&Solution {}).part2(&input);
    assert_eq!(result, 0);
  }

}