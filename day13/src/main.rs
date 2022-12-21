use serde::Deserialize;
use std::cmp::Ordering;
use std::fs;

#[derive(Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum Packet {
    Array(Vec<Packet>),
    Integer(u8),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, right: &Self) -> Ordering {
        match (self, right) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::Array(a), Packet::Array(b)) => {
                for i in 0..a.len().max(b.len()) {
                    // Left ran out of items
                    if i >= a.len() {
                        return Ordering::Less;
                    }
                    // Right ran out of items
                    if i >= b.len() {
                        return Ordering::Greater;
                    }
                    // If values are equal, consider next pair
                    match a[i].cmp(&b[i]) {
                        Ordering::Equal => (),
                        ordering => return ordering,
                    }
                }
                Ordering::Equal
            }
            (Packet::Array(_), Packet::Integer(_)) => {
                // Convert the integer into an array and compare
                self.cmp(&Packet::Array(vec![right.clone()]))
            }
            (Packet::Integer(_), Packet::Array(_)) => {
                // Convert the integer into an array and compare
                Packet::Array(vec![self.clone()]).cmp(right)
            }
        }
    }
}

fn main() {
    let raw = fs::read_to_string("./inputs/day13.txt").unwrap();

    let packets: Vec<Packet> = raw
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    println!("Part 1: {}", part_one(&packets));
    println!("Part 2: {}", part_two(&packets));
}

fn part_one(packets: &Vec<Packet>) -> usize {
    packets
        .chunks(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < pair[1])
        .fold(0, |acc, (i, _)| acc + i + 1)
}

fn part_two(packets: &Vec<Packet>) -> usize {
    // Creating an array of references allows us to do two things:
    //    1. Sort this vector separately without mutating the master packet vector
    //    2. Compare our divider packets using references instead of their interior values
    let mut packet_refs: Vec<&Packet> = packets.iter().map(|packet| packet).collect();

    let divider_a = Packet::Array(vec![Packet::Array(vec![Packet::Integer(2)])]);
    let divider_b = Packet::Array(vec![Packet::Array(vec![Packet::Integer(6)])]);

    packet_refs.push(&divider_a);
    packet_refs.push(&divider_b);

    packet_refs.sort();

    let idx_a = 1 + packet_refs
        .iter()
        .position(|&packet| packet == &divider_a)
        .unwrap();
    let idx_b = 1 + packet_refs
        .iter()
        .position(|&packet| packet == &divider_b)
        .unwrap();

    idx_a * idx_b
}
