use rdev::{listen, Event, EventType, Key};
use std::{
    collections::HashSet,
    thread::{sleep, spawn},
    time::Duration,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
    KEYEVENTF_KEYUP, MAP_VIRTUAL_KEY_TYPE, VIRTUAL_KEY, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7,
    VK_8, VK_A, VK_D, VK_E, VK_F, VK_G, VK_H, VK_I, VK_O, VK_P, VK_Q, VK_R, VK_S, VK_T, VK_U, VK_V,
    VK_W, VK_Y,
};

fn set_key(key: VIRTUAL_KEY, down: bool) {
    let array = [INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                wScan: unsafe {
                    MapVirtualKeyA(key.0.try_into().unwrap(), MAP_VIRTUAL_KEY_TYPE(0))
                }
                .try_into()
                .unwrap(),
                dwFlags: if down {
                    KEYBD_EVENT_FLAGS::default()
                } else {
                    KEYEVENTF_KEYUP
                },
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }];
    let return_value =
        unsafe { SendInput(&array, std::mem::size_of::<INPUT>().try_into().unwrap()) };
    assert_eq!(return_value, array.len().try_into().unwrap());
}

fn press_key(keys_down: &mut HashSet<u16>, key: VIRTUAL_KEY) {
    keys_down.insert(key.0);
    set_key(key, true);
}

fn release_key(keys_down: &mut HashSet<u16>, key: VIRTUAL_KEY) {
    set_key(key, false);
    keys_down.remove(&key.0);
}

fn release_all_keys(keys_down: &mut HashSet<u16>) {
    for key in keys_down.drain() {
        set_key(VIRTUAL_KEY(key), false);
    }
}

fn main() {
    spawn(move || {
        loop {
            if let Err(error) = listen(callback) {
                println!("Error: {:?}", error)
            }
        }

        fn callback(event: Event) {
            match event.event_type {
                EventType::KeyPress(Key::BackSlash) => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });

    let ram;
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

    let mut keys_down: HashSet<u16> = HashSet::new();

    sleep(Duration::from_secs(4));
    for (address, byte) in ram.into_iter().enumerate() {
        release_key(&mut keys_down, VK_V);
        release_all_keys(&mut keys_down);
        sleep(Duration::from_millis(17));

        let address: u16 = address.try_into().unwrap();
        for bit in 0..u16::BITS as usize {
            const KEYS: [VIRTUAL_KEY; 16] = [
                VK_Q, VK_W, VK_E, VK_R, VK_T, VK_Y, VK_U, VK_I, VK_O, VK_P, VK_A, VK_S, VK_D, VK_F,
                VK_G, VK_H,
            ];
            if (address & (1 << (15 - bit))) != 0 {
                press_key(&mut keys_down, KEYS[bit]);
            }
        }

        for bit in 0..u8::BITS as usize {
            const KEYS: [VIRTUAL_KEY; 8] = [VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8];
            if (byte & (1 << (7 - bit))) != 0 {
                press_key(&mut keys_down, KEYS[bit]);
            }
        }

        sleep(Duration::from_millis(17));

        press_key(&mut keys_down, VK_V);

        sleep(Duration::from_millis(17));
    }
}
