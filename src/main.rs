use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

const NUM_WORDS: usize = 2048;
const NUM_REGISTERS: usize = 16;
const PC_REGISTER: usize = 15;
type InstImpl = fn(u8, u8, u8, &mut [u32; NUM_REGISTERS], &mut [u32; NUM_WORDS]) -> bool;

fn main() {
    // Create environment
    // Initialise 2kb of memory
    let mut memory: [u32; NUM_WORDS] = [0; NUM_WORDS];
    let mut registers: [u32; NUM_REGISTERS] = [0; NUM_REGISTERS];
    // Read program into environment
    // TOOD(td): Read from args
    let f = match File::open("program.arm") {
        Ok(file) => file,
        Err(_) => {
            panic!("Unable to open file");
        }
    };
    let mut i = 0;
    let file = BufReader::new(&f);
    for line in file.lines() {
        let l = line.unwrap();
        let encoded = encode(l.clone());
        memory[i] = encoded;
        i += 1;
    }
    // Start environment
    loop {
        // Fetch
        let ins = memory[registers[PC_REGISTER] as usize];
        // Decode
        // TODO(td): Instruction decoding
        // Execute
        if !execute(ins, &mut registers, &mut memory) {
            break;
        }
        // TODO(td) Make this + 4?
        registers[PC_REGISTER] = registers[PC_REGISTER] + 1;
    }
    // Test ;)
    for addr in 0 .. NUM_REGISTERS {
        println!("R{:2} = {:8X} - {:32b}", addr, registers[addr], registers[addr]);
    }
}

fn extract_bits(ins: u32, s: u32, l: u32) -> u32 {
    let mask = (1 << l) - 1;
    return (ins & (mask << s)) >> s;
}

fn nop(_: u8, _: u8, _: u8, _: &mut [u32; NUM_REGISTERS], _: &mut [u32; NUM_WORDS]) -> bool {
    return false;
}

fn add(dest: u8, arg1: u8, arg2: u8, registers: &mut [u32; NUM_REGISTERS], _: &mut [u32; NUM_WORDS]) -> bool {
    registers[dest as usize] = val(arg1, registers) + val(arg2, registers);
    return true;
}

fn sub(dest: u8, arg1: u8, arg2: u8, registers: &mut [u32; NUM_REGISTERS], _: &mut [u32; NUM_WORDS]) -> bool {
    let res = (val(arg1, registers) as i32) - (val(arg2, registers) as i32);
    registers[dest as usize] = res as u32;
    return true;
}

static INSTRUCTIONS: [InstImpl; 3] = [nop, add, sub];

fn execute(ins: u32, registers: &mut [u32; NUM_REGISTERS], memory: &mut [u32; NUM_WORDS]) -> bool {
    if ins == 0 {
        return false;
    }
    let op = extract_bits(ins, 27, 5) as usize;
    let reg = extract_bits(ins, 16, 4) as u8;
    let arg1 = extract_bits(ins, 8, 8) as u8;
    let arg2 = extract_bits(ins, 0, 8) as u8;
    return INSTRUCTIONS[op](reg, arg1, arg2, registers, memory);
}

fn val(arg: u8, registers: &[u32; NUM_REGISTERS]) -> u32 {
  return match arg & 128 {
      128 => registers[(arg & 127) as usize] as u32,
      _   => arg as u32,
  };
}

/**
 * Encodes an instruction such as "ADD R1 R2" into a 
 * 32 bit word (5 bit op code, 5 bit condition, 1 4 bit register, 2x 8 bit args).
 * <OP-><CON>XX<RG><-REG2-><-REG3->
 * 0000100000XX00010000000400000005 = ADD R1 #4 #5
 * TODO: 
 * - Add more op codes
 * - Add conditional op code support
 * - Add register support (have a flag & make literals < 128)
 */
fn encode(instruction: String) -> u32 {
    // Split the string by whitespace
    let mut tokens = instruction.split_whitespace();
    // Find the op code
    let op = match tokens.next() {
        Some("ADD") => 1,         // ADD R1 R2 R3
        Some("SUB") => 2,         // SUB R1 R2 R3
        Some(_) | None => 0,      // NO-OP
    };
    // Don't implement condition codes yet
    let cond = 0;
    // For each argument, encode that too
    let mut args = 0;
    let mut i = 2;
    while i >= 0 {
        let literal: &str = match tokens.next() {
            Some(x) => x,
            None => break,
        };
        let literal: u32 = match literal.parse() {
            Ok(num) if num < 256 => num,
            Ok(_) => {
                // Only support 8 bit positive integers
                0
            },
            Err(_) => {
                let mut reg = parse_register(literal);
                // The value is 1000XXXX if it's an 8 bit register
                // or YYYY if it's a register
                if i < 2 {
                    reg |= 1 << 7;
                }
                reg & 255
            },
        };
        if i == 2 {
            // There are only 16 registers, so AND with 1111
            args |= (literal & 15) << (i * 8);
        } else {
            args |= literal << (i * 8);
        }
        i -= 1;
    }
    return validate(op << (27)) | cond << 22 | args;
}

fn parse_register(token: &str) -> u32{
    let bytes = token.as_bytes();
    return match bytes.get(0) {
        // Char code for 'R'
        Some(&82) => {
            let s = bytes.split_at(1).1;
            return match String::from_utf8_lossy(&s).parse() {
                Ok(num) if num < 16 => num,
                _ => 0,
            };
        },
        _ => {
            // Throw
            return 0;
        },
    };
}

fn validate(instruction: u32) -> u32 {
    // TODO: Validate the instruction here (fail for assembly error).
    instruction
}
