use crate::cpu::Cpu;
use crate::instructions::{ Instruction, extract_opcode, decode_instruction };

impl Cpu {
    fn disassemble_j_type(&self, instruction: u32, opcode: u32) -> String {
        let instr = decode_instruction(instruction);
        match instr {
            Instruction::JType { address } => {
                match opcode {
                    0x02 => format!("j     0x{:08x}", address),
                    0x03 => format!("jal   0x{:08x}", address),
                    _ => format!("Unknown J-type instruction with opcode {:#x}", opcode),
                }
            },
            _ => unreachable!()
        }
    }

    fn disassemble_i_type(&self, instruction: u32, opcode: u32) -> String {
        let instr = decode_instruction(instruction);
        match instr {
            Instruction::IType { rs, rt, immediate } => {
                match opcode {
                    0x08 => format!("addi  R{}, R{}, {}", rt, rs, immediate as i16),
                    0x0C => format!("andi  R{}, R{}, {}", rt, rs, immediate),
                    0x0D => format!("ori   R{}, R{}, {}", rt, rs, immediate),
                    0x0A => format!("slti  R{}, R{}, {}", rt, rs, immediate as i16),
                    0x23 => format!("lw    R{}, {}(R{})", rt, immediate as i16, rs),
                    0x2B => format!("sw    R{}, {}(R{})", rt, immediate as i16, rs),
                    _ => format!("Unknown I-type instruction with opcode {:#x}", opcode),
                }
            },
            _ => unreachable!()
        }
    }
    fn disassemble_r_type(&self, instruction: u32) -> String {
        let instr = decode_instruction(instruction);
        match instr {
            Instruction::RType { rs, rt, rd, shamt, func } => {
                match func {
                    0x20 => format!("add   R{}, R{}, R{}", rd, rs, rt),
                    0x22 => format!("sub   R{}, R{}, R{}", rd, rs, rt),
                    0x24 => format!("and   R{}, R{}, R{}", rd, rs, rt),
                    0x25 => format!("or    R{}, R{}, R{}", rd, rs, rt),
                    0x2A => format!("slt   R{}, R{}, R{}", rt, rs, rt),
                    _ => format!("Unknown RType instruction with func code: {:#x}", func),
                }
            },
            _ => unreachable!()
        }
    }

    pub fn disassemble(&self, instruction: u32) -> String {
        let opcode = extract_opcode(instruction);

        match opcode {
            0x00 => self.disassemble_r_type(instruction),
            0x02 | 0x03 => self.disassemble_j_type(instruction, opcode),
            _ => self.disassemble_i_type(instruction, opcode),
        }
    }
}