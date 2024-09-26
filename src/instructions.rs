#[derive(Debug)]
pub enum Instruction {
    RType { rs: usize, rt: usize, rd: usize, shamt: u32, func: u32 },
    IType { rs: usize, rt: usize, immediate: u32 },
    JType { address: u32 },
    Unknown,
}

pub fn extract_opcode(instruction: u32) -> u32 {
    (instruction >> 26) & 0x3F
}

fn decode_r_type(instruction: u32) -> Instruction {
    let rs = ((instruction >> 21) & 0x1F) as usize;
    let rt = ((instruction >> 16) & 0x1F) as usize;
    let rd = ((instruction >> 11) & 0x1F) as usize;
    let shamt = (instruction >> 6) & 0x1F;
    let func = instruction & 0x3F;

    Instruction::RType { rs, rt, rd, shamt, func }
}

fn decode_i_type(instruction: u32) -> Instruction {
    let rs = ((instruction >> 21) & 0x1F) as usize;
    let rt = ((instruction >> 16) & 0x1F) as usize;
    let immediate = instruction & 0xFFFF;

    Instruction::IType { rs, rt, immediate }
}

fn decode_j_type(instruction: u32) -> Instruction {
    let address = instruction & 0x3FFFFFFF;

    Instruction::JType { address }
}

pub fn decode_instruction(instruction: u32) -> Instruction {
    let opcode = extract_opcode(instruction);

    match opcode {
        0x00 => decode_r_type(instruction),
        0x02 | 0x03 => decode_j_type(instruction),
        _ => decode_i_type(instruction),
    }
}
