use crate::cpu::Cpu;

impl Cpu {
    pub fn debug_registers(&self, register_len: usize) {
        for i in 0..register_len {
            println!("R{}: 0x{:08x}", i, self.read_register(i));
        }
        println!("HI: 0x{:08x}", self.get_hi());
        println!("LO: 0x{:08x}", self.get_lo());
        println!("PC: 0x{:08x}", self.get_pc());
    }

    pub fn debug_memory(&self, start: u32, end: u32) {
        for addr in (start..end).step_by(4) {
            println!("0x{:08x}: 0x{:08x}", addr, self.read_word(addr));
        }
    }
}
