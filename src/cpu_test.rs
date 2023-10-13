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
    fn ccf() {
        let mut cpu = CPU::new();
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.subtract = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x3F);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn cp() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x02;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xB8);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn cp_hl() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.set_hl(0x02);
        cpu.memory_bus.write_byte(0x0000, 0xBE);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.program_counter, 0x0002);
    }
    #[test]
    fn cp_zero() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.registers.b = 0x01;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xB8);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn cpl() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x00;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x2F);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0xFF);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, true);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn inc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x00;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x3C);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn dec() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x3D);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn dec_zero() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x01;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x3D);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn rl() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b1000_0000;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x10);

        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }
    #[test]
    fn rr() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x18);

        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn rla() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1000_0000;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x17);
        cpu.memory_bus.write_byte(0x0001, 0x10);

        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn rra() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x1F);
        cpu.memory_bus.write_byte(0x0001, 0x10);

        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn rlc() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b1000_0000;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x00);
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0001);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }
    #[test]
    fn rrc() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x08);
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b1000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn rrca() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x0F);
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b1000_0000);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, false);
    }

    #[test]
    fn scf() {
        let mut cpu = CPU::new();
        cpu.registers.f.carry = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.subtract = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x37);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.f.carry, true);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn sla() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b1000_0000;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x20);
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn sra() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x28);
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
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
    fn interupt_joy() {
        let mut cpu = CPU::new();
        cpu.program_counter = 0x0000;
        cpu.interupt_master_enable = true;
        cpu.memory_bus.write_byte(0x0000, 0xFB);
        cpu.memory_bus.interupt_flags.joypad = true;
        cpu.memory_bus.interupt_enable.joypad = true;
        // Stop instruction at interupt vector
        cpu.memory_bus.write_byte(0x0060, 0x10);
        cpu.memory_bus.write_byte(0x0061, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.program_counter, 0x0061);
        assert_eq!(cpu.interupt_master_enable, true);
    }

    #[test]
    fn pop_bc() {
        let mut cpu = CPU::new();
        cpu.registers.set_de(0x0003);
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xD5);
        cpu.memory_bus.write_byte(0x0001, 0xC1);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.get_bc(), 0x0003);
    }

    #[test]
    fn push_bc() {
        let mut cpu = CPU::new();
        cpu.registers.set_bc(0x0003);
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xC5);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
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
        assert_eq!(
            cpu.memory_bus.read_byte(cpu.registers.get_hl()),
            0b0000_1011
        );
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
        assert_eq!(
            cpu.memory_bus.read_byte(cpu.registers.get_hl()),
            0b0000_0011
        );
        assert_eq!(cpu.program_counter, 0x0003);
    }
  
    #[test]
    fn xor_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000_0011;
        cpu.registers.c = 0b0000_0010;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xA9);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0001);
        assert_eq!(cpu.program_counter, 0x0002);
    }
  
    #[test]
    fn or_c() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000_0001;
        cpu.registers.c = 0b0000_0010;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xA9);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.a, 0b0000_0011);
        assert_eq!(cpu.program_counter, 0x0002);
    }
  
    #[test]
    fn swap() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b0000_0001;

        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x30);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0001_0000);
        assert_eq!(cpu.program_counter, 0x0003);
    }
      
    #[test]
    fn sbc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x02;
        cpu.registers.b = 0x01;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x98);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);

        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn sub() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0x02;
        cpu.registers.b = 0x01;

        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x90);
        // Stop instruction
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);

        assert_eq!(cpu.registers.a, 0x01);
        assert_eq!(cpu.program_counter, 0x0002);
    }

    #[test]
    fn srl() {
        let mut cpu = CPU::new();
        cpu.registers.b = 0b0000_0001;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0xCB);
        cpu.memory_bus.write_byte(0x0001, 0x38);

        cpu.memory_bus.write_byte(0x0002, 0x10);
        cpu.run(4.194304);
        assert_eq!(cpu.registers.b, 0b0000_0000);
        assert_eq!(cpu.registers.f.zero, true);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

    #[test]
    fn daa()
    {
        let mut cpu = CPU::new();
        //0x60 + 0x60 = 0xc0
        cpu.registers.a = 0xc0;
        cpu.registers.f.carry = true;
        cpu.program_counter = 0x0000;
        cpu.memory_bus.write_byte(0x0000, 0x27);
    
        cpu.memory_bus.write_byte(0x0001, 0x10);
        cpu.run(4.194304);
        //0x90 + 0x90 = 0x120 (0x20 + carry)
        assert_eq!(cpu.registers.a, 0x20);
        assert_eq!(cpu.registers.f.zero, false);
        assert_eq!(cpu.registers.f.subtract, false);
        assert_eq!(cpu.registers.f.half_carry, false);
        assert_eq!(cpu.registers.f.carry, true);
    }

}
