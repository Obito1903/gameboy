#[cfg(test)]
mod cpu_tests {
    use crate::cpu::CPU;

    #[test]
    fn add_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.c = 0x02;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x81);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x03);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn addhl_bc() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x01);
        cpu.registers.set_bc(0x02);
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x09);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.get_hl(), 0x03);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn add_d8() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xC6);
        cpu.memory_bus.write_byte(0x0001, 0x02);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x03);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn adc_b() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x02;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x88);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x04);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn adc_d8() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCE);
        cpu.memory_bus.write_byte(0x0001, 0x02);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x04);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn and_d() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000_0011;
        cpu.registers.d = 0b0000_0010;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xA2);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0010);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn and_d8() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xE6);
        cpu.memory_bus.write_byte(0x0001, 0x02);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn bit_6_h() {
        let mut cpu = CPU::new();
        cpu.registers.h = 0b0100_0000;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x74);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn bit_4_b() {
        let mut cpu = CPU::new();
        cpu.registers.h = 0b0100_0000;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x40);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn call_nn() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCD);
        cpu.memory_bus.write_byte(0x0001, 0x00);
        cpu.memory_bus.write_byte(0x0002, 0x10);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0010, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.program_counter, 0x0011);
        assert_eq!(cpu.stack_pointer, 0xFFFD);
        assert_eq!(cpu.memory_bus.read_word(0xFFFD), 0x0003);
    }

    #[test]
    fn set_2_c() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x03;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0xD1);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.c, 0x07);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn set_3_hl() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x03);
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0xDE);
        cpu.memory_bus.write_byte(0x03, 0b0000_0011);

        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.memory_bus.read_byte(cpu.registers.get_hl()), 0b0000_1011);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn res_1_c() {
        let mut cpu = CPU::new();
        cpu.registers.c = 0x03;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x89);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.c, 0x01);
        assert_eq!(cpu.program_counter, 0x0003);
    }

    #[test]
    fn res_2_hl() {
        let mut cpu = CPU::new();
        cpu.registers.set_hl(0x07);
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x96);
        cpu.memory_bus.write_byte(0x07, 0b0000_0111);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.memory_bus.read_byte(cpu.registers.get_hl()), 0b0000_0011);
        assert_eq!(cpu.program_counter, 0x0003);
    }
}
