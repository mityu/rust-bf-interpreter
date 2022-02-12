mod interpreter {
    enum Op {
        Increment,
        Decrement,
        ShiftLeft,
        ShiftRight,
        PrintChar,
        GetChar,
        LoopStart,
        LoopEnd
    }

    impl Op {
        #[allow(dead_code)]
        fn to_string(&self) -> String {
            return match self {
                Op::Increment => String::from("Increment"),
                Op::Decrement => String::from("Decrement"),
                Op::ShiftLeft => String::from("ShiftLeft"),
                Op::ShiftRight => String::from("ShiftRight"),
                Op::PrintChar => String::from("PrintChar"),
                Op::GetChar => String::from("GetChar"),
                Op::LoopStart => String::from("LoopStart"),
                Op::LoopEnd => String::from("LoopEnd"),
            };
        }
    }

    enum Instruction {
        Increment,
        Decrement,
        ShiftLeft,
        ShiftRight,
        PrintChar,
        GetChar,
        Loop(Vec<Instruction>),
    }

    pub struct Interpreter {
        source: String,
        ops: Vec<Op>,
        inst: Vec<Instruction>,
    }

    pub fn new(s: String) -> Interpreter {
        return Interpreter{
            source: s,
            ops: Vec::<Op>::new(),
            inst: Vec::<Instruction>::new(),
        };
    }

    #[allow(dead_code)]
    fn print_instruction(v: &Vec<Instruction>) {
        print_instruction_with_indent(v, 0);
    }

    #[allow(dead_code)]
    fn print_instruction_with_indent(v: &Vec<Instruction>, depth: u8) {
        let mut indent = String::from("");
        for _ in 0..depth {
            indent.push_str("  ");
        }

        for e in v.iter() {
            let label = match e {
                Instruction::Increment =>  Some(String::from("Increment")),
                Instruction::Decrement =>  Some(String::from("Decrement")),
                Instruction::ShiftLeft =>  Some(String::from("ShiftLeft")),
                Instruction::ShiftRight => Some(String::from("ShiftRight")),
                Instruction::PrintChar =>  Some(String::from("PrintChar")),
                Instruction::GetChar =>  Some(String::from("GetChar")),
                Instruction::Loop(child) => {
                    println!("{}Loop:", indent);
                    print_instruction_with_indent(child, depth + 1);
                    None
                }
            };

            match label {
                Some(v) => println!("{}{}", indent, v),
                None => (),
            }
        }
    }

    impl Interpreter {
        pub fn run(&mut self) -> Result<(), String> {
            self.validate()?;
            self.lex_code();
            self.build_instruction();
            self.eval_instruction();
            Ok(())
        }
        fn validate(&self) -> Result<(), String> {
            let mut count = 0;
            for c in self.source.chars() {
                match c {
                    '[' => count += 1,
                    ']' => count -= 1,
                    _ => (),
                }
                if count < 0 {
                    return Err(String::from("Invalid source."));
                }
            }
            if count != 0 {
                return Err(String::from("Invalid source."));
            }
            Ok(())
        }
        fn lex_code(&mut self) {
            for c in self.source.chars() {
                let op = match c {
                    '+' => Some(Op::Increment),
                    '-' => Some(Op::Decrement),
                    '<' => Some(Op::ShiftLeft),
                    '>' => Some(Op::ShiftRight),
                    '.' => Some(Op::PrintChar),
                    ',' => Some(Op::GetChar),
                    '[' => Some(Op::LoopStart),
                    ']' => Some(Op::LoopEnd),
                    _ => None,
                };
                match op {
                    Some(v) => self.ops.push(v),
                    None => (),
                }
            }
        }
        fn build_instruction(&mut self) {
            let mut queue = Vec::<Vec::<Instruction>>::new();
            let mut inst = Vec::<Instruction>::new();
            for op in self.ops.iter() {
                let i = match op {
                    Op::Increment => Some(Instruction::Increment),
                    Op::Decrement => Some(Instruction::Decrement),
                    Op::ShiftLeft => Some(Instruction::ShiftLeft),
                    Op::ShiftRight => Some(Instruction::ShiftRight),
                    Op::PrintChar => Some(Instruction::PrintChar),
                    Op::GetChar => Some(Instruction::GetChar),
                    Op::LoopStart => {
                        queue.push(inst);
                        inst = Vec::<Instruction>::new();
                        None
                    }
                    Op::LoopEnd => {
                        let mut v = queue.pop().unwrap();
                        v.push(Instruction::Loop(inst));
                        inst = v;
                        None
                    }
                };
                match i {
                    Some(v) => inst.push(v),
                    None => (),
                }
            }
            self.inst = inst;
        }
        fn eval_instruction(&self) {
            let mut memory : Vec<u8> = vec![0];
            let mut adress : usize = 0;

            self.eval_liner(&self.inst, &mut memory, &mut adress);
        }
        fn eval_liner(
            &self,
            inst: &Vec<Instruction>,
            memory: &mut Vec<u8>,
            adress: &mut usize) {
            for op in inst.iter() {
                match op {
                    Instruction::Increment => {
                        memory[*adress] += 1
                    }
                    Instruction::Decrement => {
                        memory[*adress] -= 1
                    }
                    Instruction::ShiftLeft => {
                        if *adress == 0 {
                            memory.insert(0, 0);
                        } else {
                            *adress -= 1;
                        }
                    }
                    Instruction::ShiftRight => {
                        *adress += 1;
                        if *adress == memory.len() {
                            memory.push(0);
                        }
                    }
                    Instruction::PrintChar => {
                        print!("{}", memory[*adress] as char);
                    }
                    Instruction::GetChar => {
                        let mut input = String::new();
                        match std::io::stdin().read_line(&mut input) {
                            Ok(_) => (),
                            Err(msg) => panic!("{}", msg),
                        }
                        memory[*adress] = input.as_bytes()[0];
                    }
                    Instruction::Loop(inst) => {
                        if memory[*adress] != 0 {
                            loop {
                                self.eval_liner(inst, memory, adress);
                                if memory[*adress] == 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), String> {
    let args : Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] == "-h" || args[1] == "--help" {
        let usage = vec![
        "Usage: ./bf <argument>",
        "    argument:",
        "        <source-file>    Run brainfuck program.",
        "        -h|--help        Show this help"
        ];
        for s in usage.iter() {
            println!("{}", s);
        }
        return Ok(());
    }

    let sourcefile = &args[1];
    let source = match std::fs::read_to_string(sourcefile) {
        Ok(s) => s,
        Err(msg) => {
            println!("Error occured while reading a file: {}", sourcefile);
            println!("{}", msg);
            std::process::exit(1);
        }
    };

    let mut interpreter = interpreter::new(source);
    interpreter.run()?;
    Ok(())
}
