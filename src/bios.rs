use std::{path::Path, fs::File, io::{ErrorKind, Read, Error}};

pub struct Bios {
    data: Vec<u8>,
}

impl Bios {

    pub fn new(path: &Path) -> Result<Bios, Error> {

        // let start_time = Instant::now();

        let file = File::open(path)?;

        let mut data = Vec::with_capacity(BIOS_SIZE as usize);

        file.take(BIOS_SIZE).read_to_end(&mut data)?;

        // let elapsed_time = start_time.elapsed();
        //println!("The time was {}", elapsed_time.as_micros());

        if data.len() == BIOS_SIZE as usize {
            Ok( Bios {data: data})
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid bios size!"))
        }

    }

    pub fn load32(&self, offset: u32) -> u32 {
        let offset = offset as usize;

        let b0 = self.data[offset + 0] as u32;
        let b1 = self.data[offset + 1] as u32;
        let b2 = self.data[offset + 2] as u32;
        let b3 = self.data[offset + 3] as u32;

        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)

    }
}

const BIOS_SIZE: u64 = 512 * 1024;