#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Opcode {
    Continuation, // %x0
    Text,         // %x1
    Binary,       // %x2
    Close,        // %x8
    Ping,         // %x9
    Pong,         // %xA
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Opcode {
        match byte & 0x0F {
            0x0 => Opcode::Continuation,
            0x1 => Opcode::Text,
            0x2 => Opcode::Binary,
            0x8 => Opcode::Close,
            0x9 => Opcode::Ping,
            0xA => Opcode::Pong,
            _ => panic!("Invalid opcode"),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> u8 {
        match opcode {
           Opcode::Continuation => 0x0,
           Opcode::Text         => 0x1,
           Opcode::Binary       => 0x2,
           Opcode::Close        => 0x8,
           Opcode::Ping         => 0x9,
           Opcode::Pong         => 0xA,
        }
    }
}
