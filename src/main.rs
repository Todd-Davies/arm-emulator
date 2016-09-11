fn main() {
    // Open input file

    // Create environment
    
    // Read program into environment

    // Start environment

    // Test ;)
    test(String::from("ADD 1 2 3"));
    test(String::from("ADD 0 0 1"));
}

fn test(ins: String) {
    let encoded = encode(ins.clone());
    println!("Instruction: {}, output: {}", ins, encoded);
}

/**
 * Encodes an instruction such as "ADD R1 R2" into a 
 * 32 bit word (8 bit op code, 3x 8 bit args).
 *
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
        Some(_) | None => 0,      // NO-OP
    };
    // For each argument, encode that too (start at bit 1)
    let mut args = 0;
    let mut i = 2;
    while i >= 0 {
        let literal = match tokens.next() {
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
                // Don't support registers yet
                0
            },
        };
        args |= literal << (i * 8);
        i -= 1;
    }
    return validate(op << (3 * 8)) | args;
}

fn validate(instruction: u32) -> u32 {
    // TODO: Validate the instruction here, else return 0 (NOP)
    instruction
}
