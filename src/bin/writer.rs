use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};

fn main() {
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

    let mut enigo = Enigo::new();
    loop {
        //     enigo.mouse_down(MouseButton::Left);
        //     std::thread::sleep(std::time::Duration::from_millis(1000));
        //     enigo.mouse_up(MouseButton::Left);
        //     std::thread::sleep(std::time::Duration::from_millis(1000));
        enigo.key_down(Key::Q);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        enigo.key_up(Key::Q);
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    for (address, byte) in ram.into_iter().enumerate() {}
}
