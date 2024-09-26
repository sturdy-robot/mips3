use crate::cpu::Cpu;
use crate::instructions::{extract_opcode, Instruction};

impl Cpu {
    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::RType {
                rs,
                rt,
                rd,
                sa,
                func,
            } => match func {
                0x00 => self.instr_sll(rt, rd, sa),
                0x08 => self.instr_jr(rs),
                0x20 => self.instr_add(rs, rt, rd),
                0x22 => self.instr_sub(rs, rt, rd),
                0x24 => self.instr_and(rs, rt, rd),
                0x25 => self.instr_or(rs, rt, rd),
                0x2A => self.instr_slt(rs, rt, rd),
                _ => println!("Unknown RType instruction: {:#x}", func),
            },

            Instruction::IType { rs, rt, immediate } => {
                let opcode = extract_opcode(self.read_word(self.get_pc()));
                match opcode {
                    0x04 => self.instr_beq(rs, rt, immediate),
                    0x05 => self.instr_bne(rs, rt, immediate),
                    0x08 => self.instr_addi(rs, rt, immediate),
                    0x0C => self.instr_andi(rs, rt, immediate),
                    0x0D => self.instr_ori(rs, rt, immediate),
                    0x23 => self.instr_lw(rs, rt, immediate),
                    0x2B => self.instr_sw(rs, rt, immediate),
                    _ => println!("Unknown IType instruction: {:#x}", opcode),
                }
            }

            Instruction::JType { address } => {
                let opcode = extract_opcode(self.read_word(self.get_pc()));
                match opcode {
                    0x02 => self.instr_j(address),
                    0x03 => self.instr_jal(address),
                    _ => println!("Unknown JType instruction: {:#x}", opcode),
                }
            }

            Instruction::Unknown => {
                println!("Unknown instruction: {:#x}", self.read_word(self.get_pc()));
            }
        }
    }

    #[inline(always)]
    pub fn instr_add(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = self.read_register(rs).wrapping_add(self.read_register(rt));
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_addi(&mut self, rs: usize, rt: usize, immediate: u32) {
        let result = self.read_register(rs).wrapping_add(immediate);
        self.write_register(rt, result);
    }

    #[inline(always)]
    pub fn instr_andi(&mut self, rs: usize, rt: usize, immediate: u32) {
        let result = self.read_register(rs) & immediate;
        self.write_register(rt, result);
    }

    #[inline(always)]
    pub fn instr_ori(&mut self, rs: usize, rt: usize, immediate: u32) {
        let result = self.read_register(rs) | immediate;
        self.write_register(rt, result);
    }

    #[inline(always)]
    pub fn instr_slt(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = if self.read_register(rs) < self.read_register(rt) {
            1
        } else {
            0
        };
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_sltu(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = if self.read_register(rs) < self.read_register(rt) {
            1
        } else {
            0
        };
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_slti(&mut self, rs: usize, rt: usize, immediate: u32) {
        let result = if self.read_register(rs) < immediate {
            1
        } else {
            0
        };
        self.write_register(rt, result);
    }

    #[inline(always)]
    pub fn instr_sltiu(&mut self, rs: usize, rt: usize, immediate: u32) {
        let result = if self.read_register(rs) < immediate {
            1
        } else {
            0
        };
        self.write_register(rt, result);
    }

    #[inline(always)]
    pub fn instr_sub(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = self.read_register(rs).wrapping_sub(self.read_register(rt));
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_and(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = self.read_register(rs) & self.read_register(rt);
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_or(&mut self, rs: usize, rt: usize, rd: usize) {
        let result = self.read_register(rs) | self.read_register(rt);
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_sll(&mut self, rt: usize, rd: usize, sa: u32) {
        let result = self.read_register(rt) << sa;
        self.write_register(rd, result);
    }

    #[inline(always)]
    pub fn instr_lw(&mut self, rs: usize, rt: usize, immediate: u32) {
        let base = self.read_register(rs);
        let address = base.wrapping_add(immediate);
        let value = self.read_word(address);
        self.write_register(rt, value);
    }

    #[inline(always)]
    pub fn instr_sw(&mut self, rs: usize, rt: usize, immediate: u32) {
        let base = self.read_register(rs);
        let address = base.wrapping_add(immediate);
        let value = self.read_register(rt);
        self.write_word(address, value);
    }

    #[inline(always)]
    pub fn instr_beq(&mut self, rs: usize, rt: usize, immediate: u32) {
        if self.read_register(rs) == self.read_register(rt) {
            let pc = self.get_pc();
            self.set_pc(pc.wrapping_add((immediate as i16 as i32 * 4) as u32));
        }
    }

    #[inline(always)]
    pub fn instr_bne(&mut self, rs: usize, rt: usize, immediate: u32) {
        if self.read_register(rs) != self.read_register(rt) {
            let pc = self.get_pc();
            self.set_pc(pc.wrapping_add((immediate as i16 as i32 * 4) as u32));
        }
    }

    #[inline(always)]
    pub fn instr_j(&mut self, address: u32) {
        let pc = (self.get_pc() & 0xF0000000) | (address << 2);
        self.set_pc(pc);
    }

    #[inline(always)]
    pub fn instr_jr(&mut self, rs: usize) {
        let value = self.read_register(rs);
        self.instr_j(value);
    }

    #[inline(always)]
    pub fn instr_jal(&mut self, address: u32) {
        let pc = self.get_pc().wrapping_add(4);
        self.write_register(31, pc);
        self.instr_j(address);
    }

    #[inline(always)]
    pub fn instr_jalr(&mut self, rs: usize) {
        let pc = self.get_pc().wrapping_add(4);
        self.write_register(31, pc);
        self.instr_jr(rs);
    }
}
