use std::collections::HashMap;
use virtual_cpu::*;

fn main() {
    let mut ram = vec![0u8; 2usize.pow(16)];

    let mut args = std::env::args();
    _ = args.next(); // skip program name

    let Some(file_path) = args.next() else {
        eprintln!("provide the assembly file as an argument");
        return;
    };

    let mut labels = HashMap::new();
    let mut to_be_found_labels = HashMap::new();

    let mut ip = 0;
    let source = std::fs::read_to_string(file_path).unwrap();
    for line in source.lines() {
        if line.trim().is_empty() {
            continue;
        }

        if line.trim_start().starts_with('#') {
            continue; // comment
        }

        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "halt" => {
                ram[ip] = HALT << 3;
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "const" => {
                let register = validate_register(&mut tokens, "const");
                ram[ip] = CONST << 3 | register;
                ip += 1;

                let value = validate_value(
                    &mut tokens,
                    ip as _,
                    &labels,
                    &mut to_be_found_labels,
                    "const",
                );
                ram[ip] = (value >> 8) as u8;
                ip += 1;
                ram[ip] = value as u8;
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `const` instruction: {trailing}");
                }
            }
            "add" => {
                let dest = validate_register(&mut tokens, "add");
                ram[ip] = ADD << 3 | dest;
                ip += 1;
                let a = validate_register(&mut tokens, "add");
                let b = validate_register(&mut tokens, "add");
                ram[ip] = (a << 5) | (b << 2);
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "sub" => {
                let dest = validate_register(&mut tokens, "sub");
                ram[ip] = SUB << 3 | dest;
                ip += 1;
                let a = validate_register(&mut tokens, "sub");
                let b = validate_register(&mut tokens, "sub");
                ram[ip] = (a << 5) | (b << 2);
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "mult" => {
                let dest = validate_register(&mut tokens, "mult");
                ram[ip] = MULT << 3 | dest;
                ip += 1;
                let a = validate_register(&mut tokens, "mult");
                let b = validate_register(&mut tokens, "mult");
                ram[ip] = (a << 5) | (b << 2);
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "div" => {
                let dest = validate_register(&mut tokens, "div");
                ram[ip] = DIV << 3 | dest;
                ip += 1;
                let a = validate_register(&mut tokens, "div");
                let b = validate_register(&mut tokens, "div");
                ram[ip] = (a << 5) | (b << 2);
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "copy" => {
                let dest = validate_register(&mut tokens, "copy");
                ram[ip] = COPY << 3 | dest;
                ip += 1;
                let src = validate_register(&mut tokens, "copy");
                ram[ip] = src << 5;
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "load" => {
                let dest = validate_register(&mut tokens, "load");
                ram[ip] = LOAD << 3 | dest;
                ip += 1;
                let src = validate_register(&mut tokens, "load");
                ram[ip] = src << 5;
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "store" => {
                let dest = validate_register(&mut tokens, "store");
                ram[ip] = STORE << 3 | dest;
                ip += 1;
                let src = validate_register(&mut tokens, "store");
                ram[ip] = src << 5;
                ip += 1;

                if let Some(trailing) = tokens.next() {
                    panic!("Unexpected trailing tokens after `halt` instruction: {trailing}");
                }
            }
            "cond_copy" => {
                let dest = validate_register(&mut tokens, "cond_copy");
                ram[ip] = COND_COPY << 3 | dest;
                ip += 1;
                let condition = validate_register(&mut tokens, "cond_copy");
                let flag = match tokens
                    .next()
                    .unwrap_or_else(|| panic!("Expected cond_copy type flag"))
                {
                    "zero" => COND_ZERO_FLAG,
                    "non_zero" => COND_NON_ZERO_FLAG,
                    unknown => panic!("Unkown cond_copy type flag {unknown}"),
                };
                let src = validate_register(&mut tokens, "cond_copy");
                ram[ip] = (condition << 5) | (src << 2) | flag;
                ip += 1;
            }
            label if label.starts_with(':') => {
                let name = label.strip_prefix(':').unwrap();
                if labels.insert(name.to_string(), ip as _).is_some() {
                    panic!("Redefintion of '{name}' label");
                }
                if let Some(to_be_found_labels) = to_be_found_labels.remove(name) {
                    for location in to_be_found_labels {
                        ram[location as usize] = (ip >> 8) as u8;
                        ram[location as usize + 1] = ip as u8;
                    }
                }
            }
            unknown => panic!("unknown instruction name: '{unknown}'"),
        }
    }

    if let Some((name, _)) = to_be_found_labels.iter().next() {
        panic!("Coult not find label '{name}'");
    }

    std::fs::write("output.cbc", ram).unwrap();
}
