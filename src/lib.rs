use std::collections::HashMap;

pub const HALT: u8 = 0b00000;
pub const CONST: u8 = 0b00001;
pub const ADD: u8 = 0b00010;
pub const SUB: u8 = 0b00011;
pub const MULT: u8 = 0b00100;
pub const DIV: u8 = 0b00101;
pub const COPY: u8 = 0b00110;
pub const LOAD: u8 = 0b00111;
pub const STORE: u8 = 0b01000;
pub const COND_COPY: u8 = 0b01001;

pub const COND_ZERO_FLAG: u8 = 0b00;
pub const COND_NON_ZERO_FLAG: u8 = 0b01;

pub const RAM: u8 = 0b0000;
pub const ROM: u8 = 0b0001;

pub fn validate_register<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    instruction_name: &str,
) -> u8 {
    let register = tokens
        .next()
        .unwrap_or_else(|| panic!("Expected register for `{instruction_name}` instruction"))
        .strip_prefix("r")
        .unwrap_or_else(|| panic!("Expected a register for `{instruction_name}`"));
    let register = register.parse::<u8>().unwrap_or_else(|_| {
        panic!("Expected an integer in the register name for the `{instruction_name}` instruction")
    });
    if register >= 8 {
        panic!("Register index too big");
    }
    register
}

pub fn validate_value<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    ip: u16,
    labels: &HashMap<String, u16>,
    to_be_found_labels: &mut HashMap<String, Vec<u16>>,
    instruction_name: &str,
) -> u16 {
    let value = tokens
        .next()
        .unwrap_or_else(|| panic!("Expected value for `{instruction_name}` instruction"));

    if let Some(name) = value.strip_prefix(':') {
        if let Some(&location) = labels.get(name) {
            location
        } else {
            to_be_found_labels
                .entry(name.to_string())
                .or_default()
                .push(ip);
            0
        }
    } else {
        value
            .parse::<u16>()
            .or_else(|_| value.parse::<i16>().map(|value| value as u16))
            .unwrap_or_else(|_| panic!("Expected `{instruction_name}` value to fit in a u16"))
    }
}
