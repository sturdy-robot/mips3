use crate::instructions::decode_instruction;

const GPR_REGISTERS: usize = 32;

#[derive(Debug)]
pub struct Cpu {
    registers: [u32; GPR_REGISTERS],
    hi: u32,
    lo: u32,
    pc: u32,
    memory: Vec<u8>,
}

impl Cpu {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; GPR_REGISTERS],
            hi: 0,
            lo: 0,
            pc: 0,
            memory: vec![0; memory_size],
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; GPR_REGISTERS];
        self.hi = 0;
        self.lo = 0;
        self.pc = 0;
        self.memory.fill(0);
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn get_hi(&self) -> u32 {
        self.hi
    }

    pub fn get_lo(&self) -> u32 {
        self.lo
    }

    pub fn set_hi(&mut self, hi: u32) {
        self.hi = hi;
    }

    pub fn set_lo(&mut self, lo: u32) {
        self.lo = lo;
    }

    pub fn set_pc(&mut self, pc: u32) {
        self.pc = pc;
    }

    pub fn read_register(&self, reg: usize) -> u32 {
        self.registers[reg]
    }

    pub fn write_register(&mut self, reg: usize, value: u32) {
        if reg != 0 {
            self.registers[reg] = value;
        }
    }

    pub fn read_word(&self, address: u32) -> u32 {
        let addr = address as usize;
        let bytes = &self.memory[addr..addr + 4];
        u32::from_be_bytes(bytes.try_into().unwrap())
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        let addr = address as usize;
        let bytes = value.to_be_bytes();
        self.memory[addr..addr + 4].copy_from_slice(&bytes);
    }

    pub fn run(&mut self) {
        loop {
            let pc = self.get_pc();
            let instruction_word = self.read_word(self.get_pc());

            if instruction_word == 0 {
                break;
            }

            //println!("PC: {:#x}, instruction: {:#x}", pc, instruction_word);

            let instruction = decode_instruction(instruction_word);

            let disasm = self.disassemble(instruction_word);
            println!("{}", disasm);

            self.execute_instruction(instruction);
            //self.debug_registers(self.registers.len());

            let next_pc = pc.wrapping_add(4);
            self.set_pc(next_pc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_cpu() -> Cpu {
        Cpu::new(1024)
    }

    #[test]
    fn test_add() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 5);
        cpu.write_register(2, 10);
        cpu.instr_add(1, 2, 3);
        assert_eq!(cpu.read_register(3), 15);
    }

    #[test]
    fn test_sub() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 20);
        cpu.write_register(2, 5);
        cpu.instr_sub(1, 2, 3);
        assert_eq!(cpu.read_register(3), 15);
    }

    #[test]
    fn test_lw_sw() {
        let mut cpu = create_cpu();
        cpu.write_word(100, 42);
        cpu.instr_lw(0, 1, 100);
        assert_eq!(cpu.read_register(1), 42);
        cpu.instr_sw(0, 1, 200);
        assert_eq!(cpu.read_word(200), 42);
    }

    #[test]
    fn test_beq() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 5);
        cpu.write_register(2, 5);
        cpu.set_pc(0);
        cpu.instr_beq(1, 2, 4);
        assert_eq!(cpu.get_pc(), 16);
    }

    #[test]
    fn test_addi_andi_ori() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 5);
        cpu.write_register(2, 10);
        cpu.instr_addi(2, 3, 10);
        assert_eq!(cpu.read_register(3), 20);
        cpu.instr_andi(1, 4, 15);
        assert_eq!(cpu.read_register(4), 5);
        cpu.instr_ori(2, 5, 25);
        assert_eq!(cpu.read_register(5), 27);
    }

    #[test]
    fn test_slt() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 5);
        cpu.write_register(2, 10);
        cpu.instr_slt(1, 2, 3);
        assert_eq!(cpu.read_register(3), 1);
    }

    #[test]
    fn test_bne() {
        let mut cpu = create_cpu();
        cpu.write_register(1, 5);
        cpu.write_register(2, 10);
        cpu.set_pc(0);
        cpu.instr_bne(1, 2, 4);
        assert_eq!(cpu.get_pc(), 16);
    }

    #[test]
    fn test_j() {
        let mut cpu = create_cpu();
        cpu.set_pc(8);
        cpu.instr_j(16);
        assert_eq!(cpu.get_pc(), 64);
    }

    #[test]
    fn test_jr() {
        let mut cpu = create_cpu();
        cpu.write_register(2, 12);
        cpu.set_pc(8);
        cpu.instr_jr(2);
        assert_eq!(cpu.get_pc(), 48);
    }

    #[test]
    fn test_jalr() {
        let mut cpu = create_cpu();
        cpu.write_register(2, 12);
        cpu.write_register(1, 56);
        cpu.set_pc(8);
        cpu.instr_jalr(2, 31);
        assert_eq!(cpu.get_pc(), 48);
        assert_eq!(cpu.read_register(31), 12);
        cpu.instr_jalr(1, 3);
        assert_eq!(cpu.get_pc(), 224);
        assert_eq!(cpu.read_register(3), 52);
    }

    #[test]
    fn test_jal() {
        let mut cpu = create_cpu();
        cpu.set_pc(8);
        cpu.instr_jal(16);
        assert_eq!(cpu.get_pc(), 64);
        assert_eq!(cpu.read_register(31), 12);
    }

    #[test]
    fn test_program() {
        let mut cpu = create_cpu();

        cpu.memory[0..4].copy_from_slice(&0x00430820u32.to_be_bytes()); // add R1, R2, R3
        cpu.memory[4..8].copy_from_slice(&0x00222022u32.to_be_bytes()); // sub R4, R1, R2
        cpu.memory[8..12].copy_from_slice(&0x8CC50000u32.to_be_bytes()); // lw R5, 0(R6)
        cpu.memory[12..16].copy_from_slice(&0xACC50000u32.to_be_bytes()); // sw R5, 0(R6)

        cpu.write_register(2, 10); // R2 = 10
        cpu.write_register(3, 15); // R3 = 15
        cpu.write_register(5, 42); // R5 = 42
        cpu.write_register(6, 100); // R6 = 100
        cpu.write_word(100, 42); // Memory at address 100 holds 42

        assert_eq!(cpu.read_register(2), 10);
        assert_eq!(cpu.read_register(3), 15);
        assert_eq!(cpu.read_register(5), 42);
        assert_eq!(cpu.read_register(6), 100);
        assert_eq!(cpu.read_word(100), 42);

        cpu.run();

        assert_eq!(cpu.read_register(1), 25); // R1 = R2 + R3 => R1 = 10 + 15 = 25
        assert_eq!(cpu.read_register(4), 15); // R4 = R1 - R2 => R4 = 25 - 10 = 15
        assert_eq!(cpu.read_register(5), 42); // R5 = Memory at address 100
        assert_eq!(cpu.read_register(6), 100); // R6 = 100
        assert_eq!(cpu.read_word(100), 42); // Memory at address 100 still holds 42
    }
}
