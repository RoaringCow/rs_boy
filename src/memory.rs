use std::fs::File;
use std::io::{self, Read};



pub struct Memory {
    rom: Cartridge, // rom
    vram: [u8; 0x2000], // video ram
    eram: [u8; 0x2000], // external ram
    wram: [u8; 0x2000], // work ram
    echo: [u8; 0x1E00], // echo ram
    oam: [u8; 0x100], // object attribute memory
    io: [u8; 0x80], // io ports
    hram: [u8; 0x7F], // high ram
    ie: u8, // interrupt enable register
}
impl Memory {
    pub fn new(file: &str) -> Memory {
        Memory {
            rom: Cartridge::new(file),
            vram: [0; 0x2000],
            eram: [0; 0x2000],
            wram: [0; 0x2000],
            echo: [0; 0x1E00],
            oam: [0; 0x100],
            io: [0; 0x80],
            hram: [0; 0x7F],
            ie: 0,
        }

    }
    pub fn read_memory(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize], // ROM
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000], // VRAM
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000], // External RAM
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000], // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000], // Echo RAM
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => 0,// not usable
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00],// IO
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80], // High RAM
            0xFFFF => self.ie, // Interrupt Enable Register
            _ => panic!("address out of range"),


            // özel bir ram dosyası oluştur
        }
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        println!("reading memory at address: {:X}", address);
        match address {
            0x0000..=0x7FFF => self.rom.rom[address as usize] = value, // ROM
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value, // VRAM
            0xA000..=0xBFFF => self.eram[address as usize - 0xA000] = value, // External RAM
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000] = value, // RAM
            0xE000..=0xFDFF => self.wram[address as usize - 0xE000] = value, // Echo RAM
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = value,//self.gpu.read_oam(address), // OAM
            0xFEA0..=0xFEFF => (),// not usable
            0xFF00..=0xFF7F => self.io[address as usize - 0xFF00] = value,// IO
            0xFF80..=0xFFFE => self.hram[address as usize - 0xFF80] = value, // High RAM
            0xFFFF => self.ie = value, // Interrupt Enable Register
            _ => panic!("address out of range"),
        };
    }


}



pub struct Cartridge {
    pub rom: [u8; 0x8000],
    pub banks: Vec<[u8; 0x4000]> // 16kb banks
}
#[allow(dead_code)]
impl Cartridge {
    pub fn new(file: &str) -> Cartridge {
        let cartridge = Cartridge::parse_file_to_vector(file).unwrap();
        Cartridge {
            rom: cartridge[0..0x8000].try_into().unwrap(),
            banks: cartridge.chunks(0x4000).map(|chunk| chunk.try_into().unwrap()).collect(), 
        }
    
    }

    fn parse_file_to_vector(file_path: &str) -> io::Result<Vec<u8>> {
        // Open the file
        let mut file = File::open(file_path)?;

        // Read all bytes from the file
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        Ok(file_bytes)
    }

    pub fn switch_bank(&mut self, bank: u8) {
        // copy the bank into the second half of the rom
        self.rom[0x4000..0x8000].copy_from_slice(&self.banks[bank as usize]);
    }
}