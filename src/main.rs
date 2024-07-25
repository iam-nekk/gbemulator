use std::path::Path;

mod cartridge;
mod cpu;

// components:
// - Cartridge
// - CPU
// - Address bus 
// - PPU (pixel processing unit)
// - timer

#[allow(dead_code)]
struct Emulator{
    paused: bool,
    running: bool,
    program_counter: u16
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 2 {
        println!("Not enough arguments. Run: {} <rom file>", argv[0]);
        return;
    }

    let mut emulator = Emulator{paused: false, running: true, program_counter: 0};

    let rom_path = Path::new(&argv[1]);

    cartridge::cartridge_load(rom_path);

    loop {
        if emulator.running == false {return};

        // todo: emulator paused

        if !cpu::cpu_step() {
            println!("CPU has stopped.");
            return;
        }

        emulator.program_counter += 1
    }

}