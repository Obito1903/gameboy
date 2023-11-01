#[cfg(test)]
mod scren_tests {
    use crate::cpu::MemoryBus;
    use crate::cpu::CPU;
    use crate::gameboy::Gameboy;

    #[test]
    fn test_screen() {
        let mut gb = Gameboy::new();
        gb.init(format!("assets/bootroms/dmg.bin"), true, false);
        // Write $3C $7E $42 $42 $42 $42 $42 $42 $7E $5E $7E $0A $7C $56 $38 $7C to $8000-$800F
        gb.cpu.memory_bus.write_byte(0x8000, 0x3C);
        gb.cpu.memory_bus.write_byte(0x8001, 0x7E);
        gb.cpu.memory_bus.write_byte(0x8002, 0x42);
        gb.cpu.memory_bus.write_byte(0x8003, 0x42);
        gb.cpu.memory_bus.write_byte(0x8004, 0x42);
        gb.cpu.memory_bus.write_byte(0x8005, 0x42);
        gb.cpu.memory_bus.write_byte(0x8006, 0x42);
        gb.cpu.memory_bus.write_byte(0x8007, 0x42);
        gb.cpu.memory_bus.write_byte(0x8008, 0x7E);
        gb.cpu.memory_bus.write_byte(0x8009, 0x5E);
        gb.cpu.memory_bus.write_byte(0x800A, 0x7E);
        gb.cpu.memory_bus.write_byte(0x800B, 0x0A);
        gb.cpu.memory_bus.write_byte(0x800C, 0x7C);
        gb.cpu.memory_bus.write_byte(0x800D, 0x56);
        gb.cpu.memory_bus.write_byte(0x800E, 0x38);
        gb.cpu.memory_bus.write_byte(0x800F, 0x7C);

        gb.cpu.program_counter = 0x0000;
        // Stop instruction
        gb.cpu.memory_bus.write_byte(0x0001, 0x10);
        gb.run(4.194304);
    }
}
