use crate::registers::Register;

#[allow(dead_code)]
pub struct CPU {
    pub registers: Register,
    memory: [u8; 0xFFFF],
    halted: bool,
    ei: bool,

}
impl CPU {

    #[allow(dead_code)]
    pub fn new() -> CPU {
        CPU {

            registers: Register {
                a: 0x0,
                f: 0x0,
                b: 0x0,
                c: 0x0,
                d: 0x0,
                e: 0x0,
                h: 0x0,
                l: 0x0,
                sp: 0xFFFe,
                pc: 0x0,
            },
            memory: [0; 0xFFFF],
            halted: false,
            ei: false,
        }
    }

    // to make the compiler shut the fuck up
    #[allow(dead_code)]
    fn decode_register(&mut self, register: u8) -> &mut u8{
        return match register {
            0b000 => &mut self.registers.b,
            0b001 => &mut self.registers.c,
            0b010 => &mut self.registers.d,
            0b011 => &mut self.registers.e,
            0b100 => &mut self.registers.h,
            0b101 => &mut self.registers.l,
            0b111 => &mut self.registers.a,
            _ => {panic!("this register does not exist!")}
        };

    }

    #[allow(dead_code)]
    fn fetch_instruction() {
        todo!();
    }

    fn jump_8bitoffset(&mut self) {
        let offset = self.memory[(self.registers.pc + 1) as usize] as i16;
        self.registers.pc = (self.registers.pc as i16 + offset) as u16 + 1;
        // + 1 is there for value reading. It reads the next address after
        // the jump instruction to get the offset.
    }
    fn jump_16bitaddress(&mut self) {
        let lsb_address = self.memory[(self.registers.pc + 1) as usize] as u16;
        let msb_address = self.memory[(self.registers.pc + 2) as usize] as u16;
        self.registers.pc = msb_address << 8 | lsb_address;
    }

    fn call(&mut self) {
        self.registers.sp -= 1;
        self.memory[self.registers.sp as usize] = ((self.registers.pc + 3) & 0xFF) as u8;
        self.registers.sp -= 1;
        self.memory[self.registers.sp as usize] = ((self.registers.pc + 3) >> 8) as u8;
        self.jump_16bitaddress();
    }

    fn return_instruction(&mut self) {
        self.registers.sp += 1;
        let mut return_address: u16;
        return_address = self.memory[self.registers.sp as usize] as u16;
        self.registers.sp += 1;
        return_address |= (self.memory[self.registers.sp as usize] as u16) << 8;
        self.registers.sp += 1;
        self.registers.pc = return_address;
    }
    fn inc_flag_check(&mut self, value: u8) {
        let mut flag = 0;
        if value == 0 {
            flag |= 0b10000000;
        }
        if value & 0x0F == 0 {
            flag |= 0b00100000;
        }
        self.registers.f = flag;
    }

    #[allow(dead_code)]
    pub fn run_instruction(&mut self, opcode: u8) {
        // TODO? I might make these individual functions that will get called
        // from a hashmap but i dont think this part is going to affect the 
        // overall performance. I will certainly do it if the performance is
        // shit but if it isn't i won't bother. I dont know if hashmap would
        // be faster.


        let cycles: u8 = match opcode >> 6 {
            // I couldn't use a pattern in this part
            // so i will just make it manually
            0b00 => match opcode {
                0x00 => 4,
                0x10 => todo!(),

                // ----------------- Jumps ----------------------
                //Jumps with offset 8bit
                0x18 => {
                    self.jump_8bitoffset();
                    12
                },
                0x20 => {
                    // if Zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },
                0x28 => {
                    // if zero flag set
                    if self.registers.f >> 7 == 1 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }
                },
                0x30 => {
                    // if carry flag reset
                    if self.registers.f >> 4 & 1 == 0 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },
                0x38 => {
                    // if carry flag set
                    if self.registers.f >> 4 & 1 == 1 {
                        self.jump_8bitoffset();
                        12
                    }else { 8 }

                },

                // ------------------ JP [a16] ------------------
                // Jump to address 16 bit immediate value
                0xC2 => {
                    // if zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xCA => {
                    // if zero flag set 
                    if self.registers.f >> 7 == 1 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xD2 => {
                    if self.registers.f >> 4 & 1 == 0 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                },
                0xDA => {
                    if self.registers.f >> 4 & 1 == 1 {
                        self.jump_16bitaddress();
                        16
                    }else { 12 }

                }
                0xC3 => { // şişko kalp
                    self.jump_16bitaddress();
                    16
                },

                // Jump to address in HL
                0xE9 => {
                    self.registers.pc = self.registers.get_hl();
                    4
                },


                // ------------------ CALL ------------------
                // call 16 bit immediate value
                0xC4 => {
                    if self.registers.f >> 7 == 0 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xCC => {
                    if self.registers.f >> 7 == 1 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xD4 => {
                    if self.registers.f >> 4 & 1 == 0 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xDC => {
                    if self.registers.f >> 4 & 1 == 1 {
                        self.call();
                        24
                    }else { 12 }

                },
                0xCD => {
                    self.call();
                    24
                },

                // ------------------ RETURN ------------------
                0xC9 => {
                    // return without condition
                    self.return_instruction();
                    16
                },
                0xC0 => {
                    // if zero flag reset
                    if self.registers.f >> 7 == 0 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xC8 => {
                    // if zero flag set
                    if self.registers.f >> 7 == 1 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xD0 => {
                    // if carry flag reset
                    if self.registers.f >> 4 & 1 == 0 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                0xD8 => {
                    // if carry flag set
                    if self.registers.f >> 4 & 1 == 1 {
                        self.return_instruction();
                        20
                    }else { 8 }

                },
                // ------------------ ALU 8bit ------------------
                // INC r8
                0x04 => {
                    self.registers.b += 1;
                    self.inc_flag_check(self.registers.b);
                    4
                },
                0x0C => {
                    self.registers.c += 1;
                    self.inc_flag_check(self.registers.c);
                    4
                },
                0x14 => {
                    self.registers.d += 1;
                    self.inc_flag_check(self.registers.d);
                    4
                },
                0x1C => {
                    self.registers.e += 1;
                    self.inc_flag_check(self.registers.e);
                    4
                },
                0x24 => {
                    self.registers.h += 1;
                    self.inc_flag_check(self.registers.h);
                    4
                },
                0x2C => {
                    self.registers.l += 1;
                    self.inc_flag_check(self.registers.l);
                    4
                },
                0x3C => {
                    self.registers.a += 1;
                    self.inc_flag_check(self.registers.a);
                    4
                },
                0x34 => {
                    let value = self.memory[self.registers.get_hl() as usize];
                    self.memory[self.registers.get_hl() as usize] = value + 1;
                    self.inc_flag_check(value + 1);
                    12
                },

                // ------------------ ALU 16bit ------------------ 
                // INC
                0x03 => {
                    self.registers.set_bc(self.registers.get_bc() + 1);
                    8
                },
                0x13 => {
                    self.registers.set_de(self.registers.get_de() + 1);
                    8
                },
                0x23 => {
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x33 => {
                    self.registers.sp += 1;
                    8
                },

                // DEC
                0x0B => {
                    self.registers.set_bc(self.registers.get_bc() - 1);
                    8
                },
                0x1B => {
                    self.registers.set_de(self.registers.get_de() - 1);
                    8
                },
                0x2B => {
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },
                0x3B => {
                    self.registers.sp -= 1;
                    8
                },

                // ------------------ ADD HL, r16 ------------------
                // lots of repetition here but i dont care
                0x09 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_bc();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x19 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_de();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x29 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.get_hl();
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },
                0x39 => {
                    self.registers.f = self.registers.f & 0b10000000;
                    let value = self.registers.sp;
                    let sum = self.registers.get_hl() + value;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.get_hl() & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.set_hl(sum);
                    8
                },

                // -----ADD SP with 8 bit immediate value-----
                0xE8 => {
                    let value = self.memory[(self.registers.pc + 1) as usize];
                    self.registers.f = self.registers.f & 0b10000000;
                    let sum = self.registers.sp as i16 + value as i16;
                    if sum > 0xFF {
                        self.registers.f |= 0b00010000;
                    }
                    if value & 0x0F + self.registers.sp as u8 & 0x0F > 0x0F {
                        self.registers.f |= 0b00100000;
                    }
                    self.registers.sp = sum as u16;
                    16
                },


                
                // --------------------------------------------------------------
                // some load operations that are outside of the 0x40 - 0x7F range

                // ------------------ LD r8, d8 ------------------
                0x06 => {
                    // load to B
                    self.registers.b = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x0E => {
                    // add to C
                    self.registers.c = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x16 => {
                    // add to D
                    self.registers.d = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x1E => {
                    // add to E
                    self.registers.e = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x26 => {
                    // add to H
                    self.registers.h = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x2E => {
                    // add to L
                    self.registers.l = self.memory[(self.registers.pc + 1) as usize];
                    8
                },
                0x36 => {
                    // add to address HL
                    self.memory[self.registers.get_hl() as usize] = self.memory[(self.registers.pc + 1) as usize];
                    12
                },

                // LD address from 16 bit register, A
                0x02 => {
                    self.memory[self.registers.get_bc() as usize] = self.registers.a;
                    8
                },
                0x12 => {
                    self.memory[self.registers.get_de() as usize] = self.registers.a;
                    8
                },
                0x22 => {
                    // hl increment
                    self.memory[self.registers.get_hl() as usize] = self.registers.a;
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x32 => {
                    // HL decrement
                    self.memory[self.registers.get_hl() as usize] = self.registers.a;
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },

                // LD A, address from 16 bit register
                0x0A => {
                    self.registers.a = self.memory[self.registers.get_bc() as usize];
                    8
                },
                0x1A => {
                    self.registers.a = self.memory[self.registers.get_de() as usize];
                    8
                },
                0x2A => {
                    // HL increment
                    self.registers.a = self.memory[self.registers.get_hl() as usize];
                    self.registers.set_hl(self.registers.get_hl() + 1);
                    8
                },
                0x3A => {
                    // HL decrement
                    self.registers.a = self.memory[self.registers.get_hl() as usize];
                    self.registers.set_hl(self.registers.get_hl() - 1);
                    8
                },


                _ => panic!("opcode doesn't exist") 
            },

            // Load / Halt
            0b01 => {
                if opcode >> 4 == 0x7 {
                    // HALT
                    self.halted = true;
                    4
                }else {
                    // first register
                    let first = (opcode & 0b00111000) >> 3;
                    if opcode & 0xF == 0x6 || opcode & 0xF == 0xE {
                        //Load register from HL
                        *self.decode_register(first) = self.memory[self.registers.get_hl() as usize]; 
                        8
                    }else {
                        let second = opcode & 0b00000111;
                        if opcode & 0xF < 0x8 {
                            //Load register from immediate value
                            self.memory[self.registers.get_hl() as usize] = *self.decode_register(second);
                            8
                        }else {
                            //Load register from register
                            *self.decode_register(first) = *self.decode_register(second);
                            4
                        }
                    }
                }
            },

            0b10 => match opcode >> 4 & 0b0011 {
                //only get 4 and 5. bits to identify aritmetic operation 

                // ------------------ ADD/ADC ------------------
                0b00 => {
                let op_cycles;
                let mut flag = 0;
                let sum: u16;
                if opcode > 0x87 {
                    // It's 87 and not 86 because 0x87 is ADD A, A 
                    // without the carry
                    if opcode == 0x8E {
                        // Add from HL with carry
                        let value = self.memory[self.registers.get_hl() as usize];
                        sum = value as u16 + self.registers.a as u16 + (self.registers.f as u16 & 0b00010000);

                        if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                            // Set the half carry flag if the first 4 bits overflow.
                            flag |= 0b001;
                        }
                        op_cycles = 8;

                    }
                    else {
                        // get the register value to be added
                        let value = *self.decode_register(opcode & 0x07);
                        // add them up with a bigger size in order to see the carry
                        sum = self.registers.a as u16 + value as u16 + ((self.registers.f >> 4) & 1) as u16;
                        if (value & 0x0F) + self.registers.a & 0x0F + self.registers.f & 0b00010000 > 0x0F {
                            // Set the half carry flag if the first 4 bits overflow.
                            flag |= 0b001;
                        }
                        op_cycles = 4;
                    }

                }else if opcode == 0x86 {
                    // Add from HL
                    let value = self.memory[self.registers.get_hl() as usize];
                    sum = value as u16 + self.registers.a as u16;

                    if value & 0x0F + self.registers.a & 0x0F > 0x0F {
                        flag |= 0b001;
                    }
                    op_cycles = 8;

                } else {
                    // Add from Register
                    let value = *self.decode_register(opcode & 0x07);
                    sum = self.registers.a as u16 + value as u16;

                    if  & 0x0F + self.registers.a & 0x0F > 0x0F {
                        // Set the half carry flag if the first 4 bits overflow.
                        flag |= 0b001;
                    }
                    op_cycles = 4;
                }


                if sum > 0xFF {
                    flag |= 0b0001;
                }else if sum == 0x0 {
                    flag |= 0b1;
                }

                self.registers.a = sum as u8;
                self.registers.f = flag;

                op_cycles

            },

            // ------------------ SUB/SBC ------------------
            0b01 => {
                let op_cycles;
                let mut flag = 0b01000000;
                let value: u16;
                if opcode > 0x97 {
                    if opcode == 0x9E {
                        // subtract from HL with carry
                        value = self.memory[self.registers.get_hl() as usize] as u16;
                        op_cycles = 8;
                    } else {
                        // subtract from register with carry
                        value = (*self.decode_register(opcode & 0x07) + (self.registers.f >> 4) & 0b1) as u16;
                        op_cycles = 4;
                    }
                }else {
                    if opcode == 0x96 {
                        // subtract from HL
                        value = self.memory[self.registers.get_hl() as usize] as u16;
                        op_cycles = 8;
                    } else {
                        // subtract from register
                        value = *self.decode_register(opcode & 0x07) as u16;
                        op_cycles = 4;
                    }
                }

                if value > self.registers.a as u16 {
                    // set carry flag
                    flag |= 0b00010000;
                } else if value as u8 == self.registers.a {
                    // set zero flag
                    flag |= 0b10000000;
                }
                if value as u8 & 0x0F > self.registers.a & 0x0F {
                    // set half carry flag
                    flag |= 0b00100000;
                }

                // update flag
                self.registers.f = flag;
                self.registers.a -= value as u8;

                op_cycles
            },

            // ------------------ AND/XOR ------------------
            0b10 => {
                let op_cycles;
                if opcode > 0xA7 {
                    self.registers.f = 0;
                    if opcode == 0xAE {
                        // XOR HL
                        self.registers.a = self.registers.a ^ self.memory[self.registers.get_hl() as usize];
                        op_cycles = 8;
                    }else {
                        //XOR REGİSTER
                        self.registers.a = self.registers.a ^ *self.decode_register(opcode & 0x07);
                        op_cycles = 4;
                    }
                }else {
                    self.registers.f = 0b00100000;
                    if opcode == 0xA6 {
                        // AND HL
                        self.registers.a = self.memory[self.registers.get_hl() as usize] & self.registers.a;
                        op_cycles = 8;
                    }else {
                        // AND REGİSTER
                        self.registers.a = self.registers.a & *self.decode_register(opcode & 0x07);
                        op_cycles = 4;
                    }
                }

                if self.registers.a == 0 {
                    // set zero flag if zero
                    self.registers.f |= 0b10000000;
                }

                op_cycles
            },


            // ------------------ OR/CP ------------------
            0b11 => {
                let op_cycles;
                if opcode > 0xB7 {
                    self.registers.f = 0b01000000;
                    let value;

                    // Get the value. Register or [HL] in memory.
                    if opcode == 0xBE {
                        // CP HL
                        value = self.memory[self.registers.get_hl() as usize];
                        op_cycles = 8;

                    }else {
                        // CP Reg
                        value = *self.decode_register(opcode & 0x07);
                        op_cycles = 4;
                    }

                    if self.registers.a < value {
                        // set carry flag
                        self.registers.f |= 0b00010000;
                    }
                    // It will check zero even if the carry flag part was true
                    // but i dont really care there wont be any performance difference
                    if self.registers.a & 0x0f < value {
                        // set half carry flag
                        self.registers.f |= 0b00100000;
                    }else if self.registers.a == value{
                        // Set zero flag
                        self.registers.f |= 0b10000000;
                    }

                }else {
                    self.registers.f = 0;
                    if opcode == 0xB6 {
                        // OR HL
                        self.registers.a |= self.memory[self.registers.get_hl() as usize];
                        op_cycles = 8;
                    }else {    
                        // Or Reg
                        self.registers.a |= *self.decode_register(opcode & 0x07);
                        op_cycles = 4;
                    }
                    if self.registers.a == 0 {
                        self.registers.f = 0b10000000;
                    }
                }
                op_cycles
            }


            _ => 0,
        },

        0b11 => {
            todo!();
        },


        _ => 0

    };
    println!("opcode: {:x} cycles: {}", opcode, cycles);

}



}

