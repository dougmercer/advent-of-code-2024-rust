use std::{error::Error, fs};

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

fn division(value: usize, combo: usize) -> usize {
    let value = value as f64;
    let divisor = 2usize.pow(combo as u32) as f64;
    (value / divisor).floor() as usize
}

fn adv(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.registers[A] = division(device.registers[A], operand.combo(device.registers)?);
    device.increment_ip();
    Ok(())
}

fn bxl(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.registers[B] = device.registers[B] ^ operand.literal();
    device.increment_ip();
    Ok(())
}

fn bst(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.registers[B] = operand.combo(device.registers)? % 8;
    device.increment_ip();
    Ok(())
}

fn jnz(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    if device.registers[A] != 0 {
        device.ip = operand.literal();
        return Ok(());
    }
    device.increment_ip();
    Ok(())
}

fn bxc(device: &mut Device, _: Command) -> Result<(), Box<dyn Error>> {
    device.registers[B] = device.registers[B] ^ device.registers[C];
    device.increment_ip();
    Ok(())
}

fn out(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.output.push(operand.combo(device.registers)? % 8);
    device.increment_ip();
    Ok(())
}

fn bdv(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.registers[B] = division(device.registers[A], operand.combo(device.registers)?);
    device.increment_ip();
    Ok(())
}

fn cdv(device: &mut Device, operand: Command) -> Result<(), Box<dyn Error>> {
    device.registers[C] = division(device.registers[A], operand.combo(device.registers)?);
    device.increment_ip();
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Command {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl Command {
    fn literal(self) -> usize {
        self as usize
    }

    fn combo(&self, registers: [usize; 3]) -> Result<usize, Box<dyn Error>> {
        match self {
            Command::Adv => Ok(0),
            Command::Bxl => Ok(1),
            Command::Bst => Ok(2),
            Command::Jnz => Ok(3),
            Command::Bxc => Ok(registers[A]),
            Command::Out => Ok(registers[B]),
            Command::Bdv => Ok(registers[C]),
            Command::Cdv => Err("Invalid command for combo".into()),
        }
    }
}

impl TryFrom<u8> for Command {
    type Error = String;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            0 => Ok(Command::Adv),
            1 => Ok(Command::Bxl),
            2 => Ok(Command::Bst),
            3 => Ok(Command::Jnz),
            4 => Ok(Command::Bxc),
            5 => Ok(Command::Out),
            6 => Ok(Command::Bdv),
            7 => Ok(Command::Cdv),
            _ => Err(format!("Invalid opcode: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Device {
    registers: [usize; 3],
    ip: usize,
    output: Vec<usize>,
    commands: Vec<Command>,
}

impl Device {
    fn from_program(input: &str) -> Result<Self, Box<dyn Error>> {
        let (registers, commands) = parse_input(input)?;
        Ok(Device {
            registers,
            ip: 0,
            output: Vec::<usize>::new(),
            commands,
        })
    }

    fn apply(&mut self, opcode: Command, operand: Command) -> Result<(), Box<dyn Error>> {
        match opcode {
            Command::Adv => adv(self, operand),
            Command::Bxl => bxl(self, operand),
            Command::Bst => bst(self, operand),
            Command::Jnz => jnz(self, operand),
            Command::Bxc => bxc(self, operand),
            Command::Out => out(self, operand),
            Command::Bdv => bdv(self, operand),
            Command::Cdv => cdv(self, operand),
        }
    }

    fn is_halted(&self) -> bool {
        self.ip + 1 > self.commands.len()
    }

    fn next_commands(&self) -> (Command, Command) {
        (self.commands[self.ip], self.commands[self.ip + 1])
    }

    fn increment_ip(&mut self) {
        self.ip += 2;
    }

    fn execute(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.is_halted() {
            let (opcode, operand) = self.next_commands();
            self.apply(opcode, operand)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Result<([usize; 3], Vec<Command>), Box<dyn Error>> {
    let (registers_part, program_part) = input.split_once("\n\n").ok_or("uh oh")?;
    let register_values: Vec<usize> = registers_part
        .lines()
        .filter_map(|line| line.split(":").nth(1)?.trim().parse().ok())
        .collect();

    let registers = [
        *register_values.get(0).ok_or("Missing register A")?,
        *register_values.get(1).ok_or("Missing register B")?,
        *register_values.get(2).ok_or("Missing register C")?,
    ];

    let commands: Vec<Command> = program_part
        .split(':')
        .nth(1)
        .ok_or("Invalid line of program commands")?
        .split(',')
        .map(str::trim)
        .filter_map(|x| x.parse::<u8>().ok())
        .map(Command::try_from)
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok((registers, commands))
}

fn simulator(input: &str) -> Result<String, Box<dyn Error>> {
    let mut device = Device::from_program(input)?;

    device.execute()?;

    Ok(device
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

fn run_program(initial_value: usize, program: &[Command]) -> Result<Vec<Command>, Box<dyn Error>> {
    let mut device = Device {
        registers: [initial_value, 0, 0],
        ip: 0,
        output: Vec::new(),
        commands: program.to_vec(),
    };

    device.execute()?;

    // Convert output to Commands
    Ok(device
        .output
        .into_iter()
        .filter_map(|x| Command::try_from(x as u8).ok())
        .collect())
}

fn find_quine_value(program: &[Command], position: usize, current_a: usize) -> Option<usize> {
    // Try each possible octal digit
    for digit in 0..8 {
        let next_a = current_a * 8 + digit;
        let expected_output = &program[position..];

        // Run program and compare output with expected slice
        if let Ok(output) = run_program(next_a, program) {
            if output == expected_output {
                // Found complete solution
                if position == 0 {
                    return Some(next_a);
                }

                // Try to find solution for earlier position
                if let Some(solution) = find_quine_value(program, position - 1, next_a) {
                    return Some(solution);
                }
            }
        }
    }
    None
}

// Stolen from Reddit user /u/mental-chaos
// https://www.reddit.com/r/adventofcode/comments/1hg38ah/2024_day_17_solutions/m2gge90/
fn find_quine(input: &str) -> Result<usize, Box<dyn Error>> {
    let device = Device::from_program(input)?;
    let program = device.commands;

    find_quine_value(&program, program.len() - 1, 0)
        .ok_or_else(|| Box::<dyn Error>::from("No solution found"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path: &str = "data/day17.input";
    let input = fs::read_to_string(path)?;
    println!("Part 1: {:?}", simulator(&input)?);
    println!("Part 2: {:?}", find_quine(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const EXAMPLE2: &str = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"#;

    #[test]
    fn test_example() {
        assert_eq!(simulator(&EXAMPLE).unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_example2() {
        assert_eq!(find_quine(&EXAMPLE2).unwrap(), 117440);
    }
}
