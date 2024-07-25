
use core::str;
use std::{fs::File, io::{Read, Seek}, path::Path};

// Each ROM bank is always 16KB
// const ROM_BANK_SIZE: i32 = 16 * 1024;

#[allow(dead_code)]
struct RomHeader {
    entry_point: u8,
    nintendo_logo: u8,

    title: String, // 16 bytes
    new_license_code: u16,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    license_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16
}

#[allow(dead_code)]
struct Cartridge {
    filename: String,
    rom_size: u32,
    rom_data: u8,
    header: RomHeader
}

#[allow(dead_code)]
static ROM_TYPES: &'static [&'static str] = &[
    "ROM ONLY",
    "MBC1",
    "MBC1+RAM",
    "MBC1+RAM+BATTERY",
    "0x04 ???",
    "MBC2",
    "MBC2+BATTERY",
    "0x07 ???",
    "ROM+RAM 1",
    "ROM+RAM+BATTERY 1",
    "0x0A ???",
    "MMM01",
    "MMM01+RAM",
    "MMM01+RAM+BATTERY",
    "0x0E ???",
    "MBC3+TIMER+BATTERY",
    "MBC3+TIMER+RAM+BATTERY 2",
    "MBC3",
    "MBC3+RAM 2",
    "MBC3+RAM+BATTERY 2",
    "0x14 ???",
    "0x15 ???",
    "0x16 ???",
    "0x17 ???",
    "0x18 ???",
    "MBC5",
    "MBC5+RAM",
    "MBC5+RAM+BATTERY",
    "MBC5+RUMBLE",
    "MBC5+RUMBLE+RAM",
    "MBC5+RUMBLE+RAM+BATTERY",
    "0x1F ???",
    "MBC6",
    "0x21 ???",
    "MBC7+SENSOR+RUMBLE+RAM+BATTERY"
];

#[allow(non_snake_case)]
fn LICENSE_CODE(code: u16) -> &'static str {
    match code {
        0x00 => "None",
        0x01 => "Nintendo R&D1",
        0x08 => "Capcom",
        0x13 => "Electronic Arts",
        0x18 => "Hudson Soft",
        0x19 => "b-ai",
        0x20 => "kss",
        0x22 => "pow",
        0x24 => "PCM Complete",
        0x25 => "san-x",
        0x28 => "Kemco Japan",
        0x29 => "seta",
        0x30 => "Viacom",
        0x31 => "Nintendo",
        0x32 => "Bandai",
        0x33 => "Ocean/Acclaim",
        0x34 => "Konami",
        0x35 => "Hector",
        0x37 => "Taito",
        0x38 => "Hudson",
        0x39 => "Banpresto",
        0x41 => "Ubi Soft",
        0x42 => "Atlus",
        0x44 => "Malibu",
        0x46 => "angel",
        0x47 => "Bullet-Proof",
        0x49 => "irem",
        0x50 => "Absolute",
        0x51 => "Acclaim",
        0x52 => "Activision",
        0x53 => "American sammy",
        0x54 => "Konami",
        0x55 => "Hi tech entertainment",
        0x56 => "LJN",
        0x57 => "Matchbox",
        0x58 => "Mattel",
        0x59 => "Milton Bradley",
        0x60 => "Titus",
        0x61 => "Virgin",
        0x64 => "LucasArts",
        0x67 => "Ocean",
        0x69 => "Electronic Arts",
        0x70 => "Infogrames",
        0x71 => "Interplay",
        0x72 => "Broderbund",
        0x73 => "sculptured",
        0x75 => "sci",
        0x78 => "THQ",
        0x79 => "Accolade",
        0x80 => "misawa",
        0x83 => "lozc",
        0x86 => "Tokuma Shoten Intermedia",
        0x87 => "Tsukuda Original",
        0x91 => "Chunsoft",
        0x92 => "Video system",
        0x93 => "Ocean/Acclaim",
        0x95 => "Varie",
        0x96 => "Yonezawa/spal",
        0x97 => "Kaneko",
        0x99 => "Pack in soft",
        0xA4 => "Konami (Yu-Gi-Oh!)",
        _ => "Invalid code"
    }
}

pub fn cartridge_load(rom_path: &Path) -> bool{
    let rom_file = File::open(rom_path);

    let mut file_result = match rom_file{
        Ok(file_result) => file_result,
        Err(e) => panic!("{}", e)
    };
    
    // let mut rom = Vec::new();

    let _ = file_result.seek(std::io::SeekFrom::Start(0));

    let file_length = file_result.seek(std::io::SeekFrom::End(0)).unwrap();
    let _ = file_result.rewind();

    println!("The file is {} bytes long", file_length);
    // println!("Position is {:X}", file_result.stream_position().unwrap());

    let _ = file_result.seek(std::io::SeekFrom::Start(TITLE_OFFSET.try_into().unwrap()));
    // println!("Position is {:X}", file_result.stream_position().unwrap());
    let mut title_buffer = [0; 15];    
    let _ = file_result.read(&mut title_buffer[..]);
    println!(" - Title: {}", str::from_utf8(&title_buffer).unwrap());
    // println!("Position is {:X}", file_result.stream_position().unwrap());

    let _ = file_result.seek(std::io::SeekFrom::Start(NEW_LICENSE_CODE_OFFSET.try_into().unwrap()));
    // println!("Position is {:X}", file_result.stream_position().unwrap());
    let mut license_code_buffer = [0; 2];    
    let _ = file_result.read(&mut license_code_buffer[..]);
    let license_code = str::from_utf8(&license_code_buffer).unwrap();
    println!(" - License: {} {}", license_code, LICENSE_CODE(license_code.parse::<u16>().unwrap_or(0x00)));
    // println!("Position is {:X}", file_result.stream_position().unwrap());



    println!("ROM succesfully loaded!");

    return false;
}


pub const TITLE_OFFSET: usize = 0x134;
pub const NEW_LICENSE_CODE_OFFSET: usize = 0x144;
// pub const ROM_SIZE_OFFSET: usize = 0x148;
// pub const RAM_SIZE_OFFSET: usize = 0x149;




#[allow(non_camel_case_types)]
#[allow(dead_code)] // probably wont be used, cant seem to get it to work properly
enum OFFSETS {
    TITLE = 0x134,
    NEW_LICENSE_CODE = 0x144,
    ROM_SIZE = 0x148,
    RAM_SIZE = 0x149
}