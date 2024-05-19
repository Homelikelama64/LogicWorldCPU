use virtual_cpu::*;

fn main() {
    let mut ram;
    {
        let mut args = std::env::args();
        _ = args.next(); // skip program name

        let Some(file_path) = args.next() else {
            eprintln!("provide the assembled file as an argument");
            return;
        };

        ram = std::fs::read(file_path).unwrap();
        assert_eq!(ram.len(), 2usize.pow(16));
    }

    const IP: usize = 7;
    let mut registers = vec![0u16; 8];
    loop {
        let mut ip = registers[IP];
        match (ram[ip as usize] & 0b11111000) >> 3 {
            HALT => break,
            CONST => {
                let register = ram[ip as usize] & 0b00000111;
                ip += 1;
                let value0 = ram[ip as usize];
                ip += 1;
                let value1 = ram[ip as usize];
                registers[IP] = ip;

                registers[register as usize] = ((value0 as u16) << 8) | value1 as u16;
            }
            ADD => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let a_register = (second_byte & 0b11100000) >> 5;
                let b_register = (second_byte & 0b00011100) >> 2;
                registers[dest as usize] =
                    registers[a_register as usize].wrapping_add(registers[b_register as usize]);
            }
            SUB => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let a_register = (second_byte & 0b11100000) >> 5;
                let b_register = (second_byte & 0b00011100) >> 2;
                registers[dest as usize] =
                    registers[a_register as usize].wrapping_sub(registers[b_register as usize]);
            }
            MULT => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let a_register = (second_byte & 0b11100000) >> 5;
                let b_register = (second_byte & 0b00011100) >> 2;
                registers[dest as usize] = (registers[a_register as usize] as i16)
                    .wrapping_mul(registers[b_register as usize] as i16)
                    as u16;
            }
            DIV => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let a_register = (second_byte & 0b11100000) >> 5;
                let b_register = (second_byte & 0b00011100) >> 2;
                registers[dest as usize] = (registers[a_register as usize] as i16)
                    .checked_div(registers[b_register as usize] as i16)
                    .unwrap_or(0) as u16;
            }
            COPY => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let src_register = (second_byte & 0b00011100) >> 2;
                registers[dest as usize] = registers[src_register as usize];
            }
            LOAD => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let address_register = (second_byte & 0b11100000) >> 5;

                let address = registers[address_register as usize] as usize;
                let first = ram[address];
                let second = ram[address + 1];
                registers[dest as usize] = ((first as u16) << 8) | second as u16;
            }
            STORE => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let src = (second_byte & 0b11100000) >> 5;

                let address = registers[dest as usize] as usize;
                ram[address] = (registers[src as usize] >> 8) as u8;
                ram[address + 1] = registers[src as usize] as u8;
            }
            COND_COPY => {
                let dest = ram[ip as usize] & 0b00000111;
                ip += 1;
                let second_byte = ram[ip as usize];
                registers[IP] = ip;

                let condition_register = (second_byte & 0b11100000) >> 5;
                let source_register = (second_byte & 0b00011100) >> 2;
                let flag = second_byte & 0b00000011;
                let condition = match flag {
                    COND_ZERO_FLAG => registers[condition_register as usize] == 0,
                    COND_NON_ZERO_FLAG => registers[condition_register as usize] != 0,
                    COND_POSITIVE_FLAG => (registers[condition_register as usize] as i16) >= 0,
                    COND_NEGATIVE_FLAG => (registers[condition_register as usize] as i16) < 0,
                    unknown => panic!("unknown flag for conditional copy: {unknown}"),
                };
                if condition {
                    registers[dest as usize] = registers[source_register as usize];
                }
            }
            unknown => panic!("unknown instruction: {unknown:08b}"),
        }
        ip = registers[IP];
        ip += 1;
        registers[IP] = ip;
    }

    for (index, register) in registers.into_iter().enumerate() {
        println!(
            "register {index} is {register:016b} (decmal: {register} or {})",
            register as i16
        );
    }
    std::fs::write("run_output.cbc", ram).unwrap();
}
