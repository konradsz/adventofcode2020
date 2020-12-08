use std::fs;

enum StopCondition {
    Loop(i32),
    Termination(i32),
}

struct Console<'a> {
    program: Vec<&'a str>,
    pc: i32,
    accumulator: i32,
}

impl<'a> Console<'a> {
    fn new(program: Vec<&'a str>) -> Self {
        Self {
            program,
            pc: 0,
            accumulator: 0,
        }
    }

    fn execute_single_instruction(&mut self) {
        let instruction = self.program[self.pc as usize];
        let mut parts = instruction.split(' ');

        match parts.next().unwrap() {
            "acc" => self.acc(parts.next().unwrap().parse::<i32>().unwrap()),
            "jmp" => self.jmp(parts.next().unwrap().parse::<i32>().unwrap()),
            "nop" => self.nop(),
            _ => panic!("unknown instruction!"),
        }
    }

    fn run_until_loop_or_terminated(&mut self) -> StopCondition {
        let mut executed_instructions = vec![];
        loop {
            if executed_instructions.contains(&self.pc) {
                return StopCondition::Loop(self.accumulator);
            } else if self.pc as usize == self.program.len() {
                return StopCondition::Termination(self.accumulator);
            }

            executed_instructions.push(self.pc);
            self.execute_single_instruction();
        }
    }

    fn acc(&mut self, arg: i32) {
        self.accumulator += arg;
        self.pc += 1;
    }

    fn jmp(&mut self, arg: i32) {
        self.pc += arg;
    }

    fn nop(&mut self) {
        self.pc += 1;
    }
}

fn part_1(program: Vec<&str>) -> i32 {
    let mut console = Console::new(program);
    if let StopCondition::Loop(acc) = console.run_until_loop_or_terminated() {
        return acc;
    }
    unreachable!()
}

fn part_2(program: Vec<&str>) -> i32 {
    for i in 0..program.len() {
        let new_instruction = if program[i].contains("nop") {
            program[i].replace("nop", "jmp")
        } else if program[i].contains("jmp") {
            program[i].replace("jmp", "nop")
        } else {
            continue;
        };

        let mut program = program.clone();
        program[i] = &new_instruction;

        let mut console = Console::new(program);
        if let StopCondition::Termination(acc) = console.run_until_loop_or_terminated() {
            return acc;
        }
    }
    unreachable!()
}

fn main() {
    let input = fs::read_to_string("input").expect("file not found");
    let program: Vec<&str> = input.lines().collect();

    assert_eq!(part_1(program.clone()), 1586);
    assert_eq!(part_2(program.clone()), 703);
}
