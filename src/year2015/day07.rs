use core::panic;
use std::collections::HashMap;

use rand::Rng;

#[derive(Debug, Clone)]
pub enum Instruction {
    ProvideLiteral(u16, String),
    ProvideWire(String, String),
    AndLiteral(u16, String, String),
    AndWire(String, String, String),
    Or(String, String, String),
    Lshift(String, u16, String),
    Rshift(String, u16, String),
    Not(String, String),
}

pub fn run(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                [a, "AND", b, "->", to_wire] => {
                    if let Ok(a) = a.parse::<u16>() {
                        Instruction::AndLiteral(a, b.to_string(), to_wire.to_string())
                    } else {
                        Instruction::AndWire(a.to_string(), b.to_string(), to_wire.to_string())
                    }
                }
                [a, "OR", b, "->", to_wire] => {
                    Instruction::Or(a.to_string(), b.to_string(), to_wire.to_string())
                }
                [a, "LSHIFT", b, "->", to_wire] => {
                    Instruction::Lshift(a.to_string(), b.parse().unwrap(), to_wire.to_string())
                }
                [a, "RSHIFT", b, "->", to_wire] => {
                    Instruction::Rshift(a.to_string(), b.parse().unwrap(), to_wire.to_string())
                }
                ["NOT", b, "->", to_wire] => Instruction::Not(b.to_string(), to_wire.to_string()),
                [a, "->", to_wire] => {
                    if let Ok(a) = a.parse::<u16>() {
                        Instruction::ProvideLiteral(a, to_wire.to_string())
                    } else {
                        Instruction::ProvideWire(a.to_string(), to_wire.to_string())
                    }
                }
                _ => panic!("Invalid instruction: {}", line),
            },
        )
        .collect()
}

pub fn part1(input: &[Instruction]) -> u16 {
    let mut wires = HashMap::new();
    let mut stack = Vec::new();
    for instruction in input.iter() {
        if let Instruction::ProvideLiteral(literal, wire) = instruction {
            wires.insert(wire.as_str(), *literal);
        } else {
            stack.push(instruction);
        }
    }

    while !stack.is_empty() {
        let random_element = rand::thread_rng().gen_range(0..stack.len());
        let instruction = stack.remove(random_element);
        match instruction {
            Instruction::ProvideWire(from_wire, to_wire) => {
                if let Some(value) = wires.get(from_wire.as_str()) {
                    wires.insert(to_wire.as_str(), *value);
                } else {
                    stack.push(instruction);
                }
            }
            Instruction::AndLiteral(literal, from_wire, to_wire) => {
                if let Some(value) = wires.get(from_wire.as_str()) {
                    wires.insert(to_wire.as_str(), *value & *literal);
                } else {
                    stack.push(instruction);
                }
            }
            Instruction::AndWire(from_wire_a, from_wire_b, to_wire) => {
                match (
                    wires.get(from_wire_a.as_str()),
                    wires.get(from_wire_b.as_str()),
                ) {
                    (Some(from_value_a), Some(from_value_b)) => {
                        wires.insert(to_wire.as_str(), *from_value_a & *from_value_b);
                    }
                    _ => {
                        stack.push(instruction);
                    }
                }
            }

            Instruction::Or(from_wire_a, from_wire_b, to_wire) => {
                match (
                    wires.get(from_wire_a.as_str()),
                    wires.get(from_wire_b.as_str()),
                ) {
                    (Some(from_value_a), Some(from_value_b)) => {
                        wires.insert(to_wire.as_str(), *from_value_a | *from_value_b);
                    }
                    _ => stack.push(instruction),
                }
            }
            Instruction::Lshift(from_wire, shift, to_wire) => {
                if let Some(value) = wires.get(from_wire.as_str()) {
                    wires.insert(to_wire.as_str(), *value << shift);
                } else {
                    stack.push(instruction);
                }
            }
            Instruction::Rshift(from_wire, shift, to_wire) => {
                if let Some(value) = wires.get(from_wire.as_str()) {
                    wires.insert(to_wire.as_str(), *value >> shift);
                } else {
                    stack.push(instruction);
                }
            }
            Instruction::Not(from_wire, to_wire) => {
                if let Some(value) = wires.get(from_wire.as_str()) {
                    wires.insert(to_wire.as_str(), !*value);
                } else {
                    stack.push(instruction);
                }
            }
            _ => todo!(),
        }
    }

    *wires.get("a").unwrap()
}

pub fn part2(input: &[Instruction]) -> u16 {
    let a = part1(input);
    let mut new_input = Vec::new();
    for instruction in input.iter() {
        if let Instruction::ProvideLiteral(_, to_wire) = instruction {
            if to_wire == "b" {
                new_input.push(Instruction::ProvideLiteral(a, to_wire.to_string()));
            } else {
                new_input.push(instruction.clone());
            }
        } else {
            new_input.push(instruction.clone());
        }
    }
    part1(&new_input)
}
