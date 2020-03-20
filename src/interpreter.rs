use crate::state::I8080State;
use crate::dissasembler;

// use std::thread;
// use std::time::Duration;
use std::io::stdin;
use std::io::Read;


pub fn run(state: &mut I8080State) {
    use crate::dissasembler::dissasembly_around;
    use crate::write_adapter::WriteAdapter;
    use std::io;
    
    let traced = dissasembler::trace(&state.memory);
    loop {
        let mut stdout = WriteAdapter(io::stdout());
        dissasembly_around(&mut stdout, &traced, &state.memory, state.PC).unwrap();
        state.print_state();
        state.PC += interpret_opcode(state);
        stdin().read_line(&mut String::new()).unwrap();
    }
}

macro_rules! as_expr {
    ($x:expr) => { $x };
}

macro_rules! ops {
    // r1 r2 | 0b11SSSDDD
    (@rule $state:expr; ($r1:ident $r2:ident | $x:expr => $y:expr, $($tail:tt)*) -> ($($accum:tt)*) ) => {
        ops!(@rule $state; ($($tail)*) -> ( $($accum)*
            _ if $state.get_op() == $x + 0b000000 => {
                let $r1 = "B"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000001 => {
                let $r1 = "C"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000010 => {
                let $r1 = "D"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000011 => {
                let $r1 = "E"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000100 => {
                let $r1 = "H"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000101 => {
                let $r1 = "L"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b000111 => {
                let $r1 = "A"; let $r2 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b001000 => {
                let $r1 = "B"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001001 => {
                let $r1 = "C"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001010 => {
                let $r1 = "D"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001011 => {
                let $r1 = "E"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001100 => {
                let $r1 = "H"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001101 => {
                let $r1 = "L"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b001111 => {
                let $r1 = "A"; let $r2 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b010000 => {
                let $r1 = "B"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010001 => {
                let $r1 = "C"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010010 => {
                let $r1 = "D"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010011 => {
                let $r1 = "E"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010100 => {
                let $r1 = "H"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010101 => {
                let $r1 = "L"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b010111 => {
                let $r1 = "A"; let $r2 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b011000 => {
                let $r1 = "B"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011001 => {
                let $r1 = "C"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011010 => {
                let $r1 = "D"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011011 => {
                let $r1 = "E"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011100 => {
                let $r1 = "H"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011101 => {
                let $r1 = "L"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b011111 => {
                let $r1 = "A"; let $r2 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b100000 => {
                let $r1 = "B"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100001 => {
                let $r1 = "C"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100010 => {
                let $r1 = "D"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100011 => {
                let $r1 = "E"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100100 => {
                let $r1 = "H"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100101 => {
                let $r1 = "L"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b100111 => {
                let $r1 = "A"; let $r2 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b101000 => {
                let $r1 = "B"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101001 => {
                let $r1 = "C"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101010 => {
                let $r1 = "D"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101011 => {
                let $r1 = "E"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101100 => {
                let $r1 = "H"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101101 => {
                let $r1 = "L"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b101111 => {
                let $r1 = "A"; let $r2 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b111000 => {
                let $r1 = "B"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111001 => {
                let $r1 = "C"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111010 => {
                let $r1 = "D"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111011 => {
                let $r1 = "E"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111100 => {
                let $r1 = "H"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111101 => {
                let $r1 = "L"; let $r2 = "A"; $y
            },
            _ if $state.get_op() == $x + 0b111111 => {
                let $r1 = "A"; let $r2 = "A"; $y
            },
        ))
    };
    // r | 0b11SSS000
    (@rule $state:expr; ($r1:ident | $x:expr => $y:expr, $($tail:tt)*) -> ($($accum:tt)*) ) => {
        ops!(@rule $state; ($($tail)*) -> ( $($accum)*
            _ if $state.get_op() == $x + 0b000_000 => {
                let $r1 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b001_000 => {
                let $r1 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b010_000 => {
                let $r1 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b011_000 => {
                let $r1 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b100_000 => {
                let $r1 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b101_000 => {
                let $r1 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b111_000 => {
                let $r1 = "A"; $y
            },
        ))
    };
    // r 0b11000DDD
    (@rule $state:expr; ($r1:ident $x:expr => $y:expr, $($tail:tt)*) -> ($($accum:tt)*) ) => {

        ops!(@rule $state; ($($tail)*) -> ( $($accum)*
            _ if $state.get_op() == $x + 0b000 => {
                let $r1 = "B"; $y
            },
            _ if $state.get_op() == $x + 0b001 => {
                let $r1 = "C"; $y
            },
            _ if $state.get_op() == $x + 0b010 => {
                let $r1 = "D"; $y
            },
            _ if $state.get_op() == $x + 0b011 => {
                let $r1 = "E"; $y
            },
            _ if $state.get_op() == $x + 0b100 => {
                let $r1 = "H"; $y
            },
            _ if $state.get_op() == $x + 0b101 => {
                let $r1 = "L"; $y
            },
            _ if $state.get_op() == $x + 0b111 => {
                let $r1 = "A"; $y
            },
        ))
    };
    // _ if ...
    (@rule $state:expr; (_ if $x:expr => $y:expr, $($tail:tt)*) -> ($($accum:tt)*) ) => {
        ops!(@rule $state; ($($tail)*) -> ( $($accum)*
            _ if $x => { $y },
        ))
    };
    // 0b11111111
    (@rule $state:expr; ($x:expr => $y:expr, $($tail:tt)*) -> ($($accum:tt)*) ) => {
        ops!(@rule $state; ($($tail)*) -> ( $($accum)*
            $x => { $y },
        ))
    };
    // end point
    (@rule $state:expr; (_ => $y:expr) -> ($($accum:tt)*) ) => {
        as_expr!(match $state.get_op() { $($accum)* _ => $y })
    };
    (@rule $state:expr; () -> ($($accum:tt)*) ) => {
        as_expr!(match $state.get_op() { $($accum)* _ => {} })
    };
    // entry
    {$state:expr; $($tokens:tt)* } => {
        ops!(@rule $state; ($($tokens)*) -> () )
    };
}


// write the dissasembly of the opcode, return the next offset
fn interpret_opcode(state: &mut I8080State) -> u16 {
    ops!{ state;
        r1 r2 | 0b01000000 => { // MOV  r1, r2| Move register to register            | 01DDDSSS        |  5   
            //writeln!(w, "MOV  {}, {}", r1, r2)?;
            1
        },
        r 0b01110000 => { // MOV  M, r  | Move register to memory              | 01110SSS        |  7   
            //writeln!(w, "MOV  M, {}", adr, r)?;
            1
        },
        r | 0b01000110 => { // MOV  r, M  | Move memory to register              | 01DDD110        |  7   
            //writeln!(w, "MOV  {}, M  ", r, adr)?;
            1
        },
        0b01110110 => { // HLT        | Halt                                 | 01110110        |  7   
            //writeln!(w, "HLT        ")?;
            1
        },
        r | 0b00000110 => { // MVI  r     | Move immediate register              | 00DDD110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "MVI  {} {:02x}", r, immediate)?;
            2
        },
        0b00110110 => { // MVI  M     | Move immediate memory                | 00110110        | 10   
            let immediate = state.get_u8();
            //writeln!(w, "MVI  M {:02x}", immediate)?;
            2
        },
        r | 0b00000100 => { // INR  r     | Increment register                   | 00DDD100        |  5   
            //writeln!(w, "INR  {}     ", r)?;
            1
        },
        r | 0b00000101 => { // DCR  r     | Decrement register                   | 00DDD101        |  5   
            //writeln!(w, "DCR  {}     ", r)?;
            1
        },
        0b00110100 => { // INR  M     | Increment memory                     | 00110100        | 10   
            let adr = state.get_u16();
            //writeln!(w, "INR  {:04x}     ",adr)?;
            3
        },
        0b00110101 => { // DCR  M     | Decrement memory                     | 00110101        | 10   
            let adr = state.get_u16();
            //writeln!(w, "DCR  {:04x}     ",adr)?;
            3
        },
        r 0b10000000 => { // ADD  r     | Add register to A                    | 10000SSS        |  4   
            //writeln!(w, "ADD  {}     ", r)?;
            1
        },
        r 0b10001000 => { // ADC  r     | Add register to A with carry         | 10001SSS        |  4   
            //writeln!(w, "ADC  {}     ", r)?;
            1
        },
        r 0b10010000 => { // SUB  r     | Subtract register from A             | 10010SSS        |  4   
            //writeln!(w, "SUB  {}     ", r)?;
            1
        },
        r 0b10011000 => { // SBB  r     | Subtract register from A with borrow | 10011SSS        |  4   
            //writeln!(w, "SBB  {}     ", r)?;
            1
        },
        r 0b10100000 => { // ANA  r     | And register with A                  | 10100SSS        |  4   
            //writeln!(w, "ANA  {}     ", r)?;
            1
        },
        r 0b10101000 => { // XRA  r     | Exclusive Or register with A         | 10101SSS        |  4   
            //writeln!(w, "XRA  {}     ", r)?;
            1
        },
        r 0b10110000 => { // ORA  r     | Or register with A                   | 10110SSS        |  4   
            //writeln!(w, "ORA  {}     ", r)?;
            1
        },
        r 0b10111000 => { // CMP  r     | Compare register with A              | 10111SSS        |  4   
            //writeln!(w, "CMP  {}     ", r)?;
            1
        },
        0b10000110 => { // ADD  M     | Add memory to A                      | 10000110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "ADD  {:04x}     ",adr)?;
            3
        },
        0b10001110 => { // ADC  M     | Add memory to A with carry           | 10001110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "ADC  {:04x}     ",adr)?;
            3
        },
        0b10010110 => { // SUB  M     | Subtract memory from A               | 10010110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "SUB  {:04x}     ",adr)?;
            3
        },
        0b10011110 => { // SBB  M     | Subtract memory from A with borrow   | 10011110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "SBB  {:04x}     ",adr)?;
            3
        },
        0b10100110 => { // ANA  M     | And memory with A                    | 10100110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "ANA  {:04x}     ",adr)?;
            3
        },
        0b10101110 => { // XRA  M     | Exclusive Or memory with A           | 10101110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "XRA  {:04x}     ",adr)?;
            3
        },
        0b10110110 => { // ORA  M     | Or memory with A                     | 10110110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "ORA  {:04x}     ",adr)?;
            3
        },
        0b10111110 => { // CMP  M     | Compare memory with A                | 10111110        |  7   
            let adr = state.get_u16();
            //writeln!(w, "C{:04x}P  M     ",adr)?;
            3
        },
        0b11000110 => { // ADI        | Add immediate to A                   | 11000110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "ADI {:02x}", immediate)?;
            2
        },
        0b11001110 => { // ACI        | Add immediate to A with carry        | 11001110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "ACI {:02x}", immediate)?;
            2
        },
        0b11010110 => { // SUI        | Subtract immediate from A            | 11010110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "SUI {:02x}", immediate)?;
            2
        },
        0b11011110 => { // SBI        | Subtract immediate from A with borrow| 11011110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "SBI {:02x}", immediate)?;
            2
        },
        0b11100110 => { // ANI        | And immediate with A                 | 11100110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "ANI {:02x}", immediate)?;
            2
        },
        0b11101110 => { // XRI        | Exclusive Or immediate with A        | 11101110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "XRI {:02x}", immediate)?;
            2
        },
        0b11110110 => { // ORI        | Or immediate with A                  | 11110110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "ORI {:02x}", immediate)?;
            2
        },
        0b11111110 => { // CPI        | Compere immediate with A             | 11111110        |  7   
            let immediate = state.get_u8();
            //writeln!(w, "CPI {:02x}", immediate)?;
            2
        },
        0b00000111 => { // RLC        | Rotate A left                        | 00000111        |  4   
            //writeln!(w, "RLC        ")?;
            1
        },
        0b00001111 => { // RRC        | Rotate A right                       | 00001111        |  4   
            //writeln!(w, "RRC        ")?;
            1
        },
        0b00010111 => { // RAL        | Rotate A left through carry          | 00010111        |  4   
            //writeln!(w, "RAL        ")?;
            1
        },
        0b00011111 => { // RAR        | Route A right through carry          | 00011111        |  4   
            //writeln!(w, "RAR        ")?;
            1
        },
        0b11000011 => { // JMP        | Jump unconditional                   | 11000011        | 10   
            let adress = state.get_u16();
            state.PC = adress;
            0
        },
        0b11011010 => { // JC         | Jump on carry                        | 11011010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11010010 => { // JNC        | Jump on no carry                     | 11010010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11001010 => { // JZ         | Jump on zero                         | 11001010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11000010 => { // JNZ        | Jump on no zero                      | 11000010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11110010 => { // JP         | Jump on positive                     | 11110010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11111010 => { // JM         | Jump on minus                        | 11111010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11101010 => { // JPE        | Jump on parity even                  | 11101010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11100010 => { // JPO        | Jump on parity odd                   | 11100010        | 10   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11001101 => { // CALL       | Call unconditional                   | 11001101        | 17   
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11011100 => { // CC         | Call on carry                        | 11011100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11010100 => { // CNC        | Call on no tarry Call on tern        | 11010100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11001100 => { // CZ         | Call on zero                         | 11001100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11000100 => { // CNZ        | Call on no zero                      | 11000100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11110100 => { // CP         | Call on positive                     | 11110100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11111100 => { // CM         | Call on minus                        | 11111100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11101100 => { // CPE        | Call on parity even                  | 11101100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11100100 => { // CPO        | Call on parity odd                   | 11100100        | 11/17
            let adress = state.get_u16();
            if false {
                state.PC = adress;
                0
            } else { 3 }
        },
        0b11001001 => { // RET        | Return                               | 11001001        | 10   
            //writeln!(w, "RET        ")?;
            1
        },
        0b11011000 => { // RC         | Return on carry                      | 11011000        | 5/11 
            //writeln!(w, "RC         ")?;
            1
        },
        0b11010000 => { // RNC        | Return on no carry                   | 11010000        | 5/11 
            //writeln!(w, "RNC        ")?;
            1
        },
        0b11001000 => { // RZ         | Return on zero                       | 11001000        | 5/11 
            //writeln!(w, "RZ         ")?;
            1
        },
        0b11000000 => { // RNZ        | Return on no zero                    | 11000000        | 5/11 
            //writeln!(w, "RNZ        ")?;
            1
        },
        0b11110000 => { // RP         | Return on positive                   | 11110000        | 5/11 
            //writeln!(w, "RP         ")?;
            1
        },
        0b11111000 => { // RM         | Return on minus                      | 11111000        | 5/11 
            //writeln!(w, "RM         ")?;
            1
        },
        0b11101000 => { // RPE        | Return on parity even                | 11101000        | 5/11 
            //writeln!(w, "RPE        ")?;
            1
        },
        0b11100000 => { // RPO        | Return on parity odd                 | 11100000        | 5/11 
            //writeln!(w, "RPO        ")?;
            1
        },
        _ if state.get_op() & 0b11000111 == 0b11000111 => { // RST        | Restart                              | 11AAA111        | 11   
            //writeln!(w, "RST {:03b}   ", (rom[pc as usize] & 0b00111000) >> 3 )?;
            1
        },
        0b11011011 => { // IN         | Input                                | 11011011        | 10   
            let device = state.get_u8();
            //writeln!(w, "IN {:02x}", device)?;
            2
        },
        0b11010011 => { // OUT        | Output                               | 11010011        | 10   
            let device = state.get_u8();
            //writeln!(w, "OUT {:02x}", device)?;
            2
        },
        0b00000001 => { // LXI  B     | Load immediate register Pair B & C   | 00000001        | 10   
            let immediate = state.get_u16();
            //writeln!(w, "LXI  B {:04x}", immediate)?;
            3
        },
        0b00010001 => { // LXI  D     | Load immediate register pair D & E   | 00010001        | 10   
            let immediate = state.get_u16();
            //writeln!(w, "LXI  D {:04x}", immediate)?;
            3
        },
        0b00100001 => { // LXI  H     | Load immediate register pair H & L   | 00100001        | 10   
            let immediate = state.get_u16();
            //writeln!(w, "LXI  H {:04x}", immediate)?;
            3
        },
        0b00110001 => { // LXI  SP    | Load immediate stack pointer         | 00110001        | 10   
            let immediate = state.get_u16();
            //writeln!(w, "LXI  SP {:04x}", immediate)?;
            3
        },
        0b11000101 => { // PUSH B     | Push register Pair B & C on stack    | 11000101        | 11   
            //writeln!(w, "PUSH B     ")?;
            1
        },
        0b11010101 => { // PUSH D     | Push register Pair D & E on stack    | 11010101        | 11   
            //writeln!(w, "PUSH D     ")?;
            1
        },
        0b11100101 => { // PUSH H     | Push register Pair H & L on stack    | 11100101        | 11   
            //writeln!(w, "PUSH H     ")?;
            1
        },
        0b11110101 => { // PUSH PSW   | Push A and Flags on stack            | 11110001        | 11   
            //writeln!(w, "PUSH PSW   ")?;
            1
        },
        0b11000001 => { // POP  B     | Pop register pair B & C off stack    | 11000001        | 10   
            //writeln!(w, "POP  B     ")?;
            1
        },
        0b11010001 => { // POP  D     | Pop register pair D & E off stack    | 11010001        | 10   
            //writeln!(w, "POP  D     ")?;
            1
        },
        0b11100001 => { // POP  H     | Pop register pair H & L off stick    | 11100001        | 10   
            //writeln!(w, "POP  H     ")?;
            1
        },
        0b11110001 => { // POP  PSW   | Pop A and Flags off stack            | 11110001        | 10   
            //writeln!(w, "POP  PSW   ")?;
            1
        },
        0b00110010 => { // STA        | Store A direct                       | 00110010        | 13   
            let immediate = state.get_u16();
            //writeln!(w, "STA {:04x}", immediate)?;
            3
        },
        0b00111010 => { // LDA        | Load A direct                        | 00111010        | 13   
            let immediate = state.get_u16();
            //writeln!(w, "LDA {:04x}", immediate)?;
            3
        },
        0b11101011 => { // XCHG       | Exchange D & E, H & L Registers      | 11101011        | 4    
            //writeln!(w, "XCHG       ")?;
            1
        },
        0b11100011 => { // XTHL       | Exchange top of stack, H & L         | 11100011        | 18   
            //writeln!(w, "XTHL       ")?;
            1
        },
        0b11111001 => { // SPHL       | H & L to stack pointer               | 11111001        | 5    
            //writeln!(w, "SPHL       ")?;
            1
        },
        0b11101001 => { // PCHL       | H & L to program counter             | 11101001        | 5    
            //writeln!(w, "PCHL       ")?;
            1
        },
        0b00001001 => { // DAD  B     | Add B & C to H & L                   | 00001001        | 10   
            //writeln!(w, "DAD  B     ")?;
            1
        },
        0b00011001 => { // DAD  D     | Add D & E to H & L                   | 00011001        | 10   
            //writeln!(w, "DAD  D     ")?;
            1
        },
        0b00101001 => { // DAD  H     | Add H & L to H & L                   | 00101001        | 10   
            //writeln!(w, "DAD  H     ")?;
            1
        },
        0b00111001 => { // DAD  SP    | Add stack pointer to H & L           | 00111001        | 10   
            //writeln!(w, "DAD  SP    ")?;
            1
        },
        0b00000010 => { // STAX B     | Store A indirect                     | 00000010        | 7    
            //writeln!(w, "STAX B     ")?;
            1
        },
        0b00010010 => { // STAX D     | Store A Indirect                     | 00010010        | 7    
            //writeln!(w, "STAX D     ")?;
            1
        },
        0b00001010 => { // LDAX B     | Load A indirect                      | 00001010        | 7    
            //writeln!(w, "LDAX B     ")?;
            1
        },
        0b00011010 => { // LDAX D     | Load A indirect                      | 00011010        | 7    
            //writeln!(w, "LDAX D     ")?;
            1
        },
        0b00000011 => { // INX  B     | Increment B & C registers            | 00000011        | 5    
            //writeln!(w, "INX  B     ")?;
            1
        },
        0b00010011 => { // INX  D     | Increment D & E registers            | 00010011        | 5    
            //writeln!(w, "INX  D     ")?;
            1
        },
        0b00100011 => { // INX  H     | Increment H & L registers            | 00100011        | 5    
            //writeln!(w, "INX  H     ")?;
            1
        },
        0b00110011 => { // INX  SP    | Increment stack pointer              | 00110011        | 5    
            //writeln!(w, "INX  SP    ")?;
            1
        },
        0b00001011 => { // DCX  B     | Decrement B & C                      | 00001011        | 5    
            //writeln!(w, "DCX  B     ")?;
            1
        },
        0b00011011 => { // DCX  D     | Decrement D & E                      | 00011011        | 5    
            //writeln!(w, "DCX  D     ")?;
            1
        },
        0b00101011 => { // DCX  H     | Decrement H & L                      | 00101011        | 5    
            //writeln!(w, "DCX  H     ")?;
            1
        },
        0b00111011 => { // DCX  SP    | Decrement stack pointer              | 00111011        | 5    
            //writeln!(w, "DCX  SP    ")?;
            1
        },
        0b00101111 => { // CMA        | Complement A                         | 00101111        | 4    
            //writeln!(w, "CMA        ")?;
            1
        },
        0b00110111 => { // STC        | Set carry                            | 00110111        | 4    
            //writeln!(w, "STC        ")?;
            1
        },
        0b00111111 => { // CMC        | Complement carry                     | 00111111        | 4    
            //writeln!(w, "CMC        ")?;
            1
        },
        0b00100111 => { // DAA        | Decimal adjust A                     | 00100111        | 4    
            //writeln!(w, "DAA        ")?;
            1
        },
        0b00100010 => { // SHLD       | Store H & L direct                   | 00100010        | 16   
            let immediate = state.get_u16();
            //writeln!(w, "SHLD  {:04x}", immediate)?;
            3
        },
        0b00101010 => { // LHLD       | Load H & L direct                    | 00101010        | 16   
            let immediate = state.get_u16();
            //writeln!(w, "LHLD  {:04x}", immediate)?;
            3
        },
        0b11111011 => { // EI         | Enable Interrupts                    | 11111011        | 4    
            //writeln!(w, "EI         ")?;
            1
        },
        0b11110011 => { // DI         | Disable Interrupts                   | 11110011        | 4    
            //writeln!(w, "DI         ")?;
            1
        },
        0b00000000 => { // NOP        | No operation                         | 00000000        | 4    
            //writeln!(w, "NOP        ")?;
            1
        },        
        _ => {
            //writeln!(w, "<UNDEFINED>")?;
            1
        }
    }
}