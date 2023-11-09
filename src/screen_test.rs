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
        gb.cpu.memory_bus.write_byte(0x8010, 0x3C);
        gb.cpu.memory_bus.write_byte(0x8011, 0x7E);
        gb.cpu.memory_bus.write_byte(0x8012, 0x42);
        gb.cpu.memory_bus.write_byte(0x8013, 0x42);
        gb.cpu.memory_bus.write_byte(0x8014, 0x42);
        gb.cpu.memory_bus.write_byte(0x8015, 0x42);
        gb.cpu.memory_bus.write_byte(0x8016, 0x42);
        gb.cpu.memory_bus.write_byte(0x8017, 0x42);
        gb.cpu.memory_bus.write_byte(0x8018, 0x7E);
        gb.cpu.memory_bus.write_byte(0x8019, 0x5E);
        gb.cpu.memory_bus.write_byte(0x801A, 0x7E);
        gb.cpu.memory_bus.write_byte(0x801B, 0x0A);
        gb.cpu.memory_bus.write_byte(0x801C, 0x7C);
        gb.cpu.memory_bus.write_byte(0x801D, 0x56);
        gb.cpu.memory_bus.write_byte(0x801E, 0x38);
        gb.cpu.memory_bus.write_byte(0x801F, 0x7C);

        //Tile map 1
        gb.cpu.memory_bus.write_byte(0x9800, 0x00);
        gb.cpu.memory_bus.write_byte(0x9801, 0x01);
        gb.cpu.memory_bus.write_byte(0x9802, 0x02);
        gb.cpu.memory_bus.write_byte(0x9803, 0x01);

        gb.cpu.program_counter = 0x0000;
        // Stop instruction
        gb.cpu.memory_bus.write_byte(0x0001, 0x10);
        gb.run(4.194304);
    }
}
