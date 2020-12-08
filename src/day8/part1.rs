use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i64),
    Jump(i64),
    Acc(i64),
}

fn parse_instruction(i: &str) -> Instruction {
    match i.split(" ").collect::<Vec<_>>().as_slice() {
        ["nop", num] => Instruction::Nop(num.parse().unwrap()),
        ["jmp", num] => Instruction::Jump(num.parse().unwrap()),
        ["acc", num] => Instruction::Acc(num.parse().unwrap()),
        a => panic!("invalid instruction {:?}", a),
    }
}


fn part1(inp: &str) -> Result<i64, ()> {
    let instr = inp.lines().filter(|i| !i.trim().is_empty()).map(parse_instruction).collect::<Vec<_>>();

    match run_code(instr) {
        Ok(i) => Ok(i),
        Err(i) => Ok(i),
    }
}

fn run_code(code: Vec<Instruction>) -> Result<i64, i64>{
    let mut pc = 0;
    let mut acc = 0;
    
    let mut instr_had = HashSet::new();

    loop {
        if pc as usize == code.len() {
            return Ok(acc);
        }

        if !instr_had.insert(pc) {
            return Err(acc);
        }


        match code[pc as usize] {
            Instruction::Nop(_) => {
                pc += 1;
            }
            Instruction::Jump(a) => {
                pc += a;
            }
            Instruction::Acc(a) => {
                pc += 1;
                acc += a;
            }
        }
    } 
}

fn part2(inp: &str) -> Result<i64, ()> {
    let instr = inp.lines().filter(|i| !i.trim().is_empty()).map(parse_instruction).collect::<Vec<_>>();

    for i in 0..instr.len() {
        let mut code = instr.clone();

        code[i] = match &code[i] {
            Instruction::Jump(x) => Instruction::Nop(*x),
            a => a.clone(),
        };

        match run_code(code) {
            Ok(i) => return Ok(i),
            
            Err(_) => ()
        }
    }

    for i in 0..instr.len() {
        
        let mut code = instr.clone();

        code[i] = match &code[i] {
            Instruction::Nop(x) => Instruction::Jump(*x),
            a => a.clone(),
        };

        match run_code(code) {
            Ok(i) => return Ok(i),
            Err(_) => ()
        }
    }


    Err(())
}



pub fn main() {  
    println!("day 8 part 1: {:?}", part1(include_str!("input1")));
    println!("day 8 part 2: {:?}", part2(include_str!("input1")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        assert_eq!(5, part1("
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
        ").unwrap())
    }

#[test]
    fn test_example_part2() {
        assert_eq!(8, part2("
nop +1
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
        ").unwrap())
    }


    #[test]
    fn test_part1() {
        assert_eq!(1528, part1(include_str!("input1")).unwrap());
    }

    #[test]
    fn test_part2() {
        assert_eq!(640, part2(include_str!("input1")).unwrap());
    }
}



