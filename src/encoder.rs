//! Luby Transform encoder
extern crate rand;

use rand::{Rng, StdRng};

use Header;
use Packet;
use RawEncodedBlock;


const BLOCK_SIZE: u64 = 1024; // Bytes

struct Source {
    rng: StdRng,
    data: Vec<u8>,
}

impl Source {
    fn new() -> Source {
        Source {
            rng: StdRng::new().unwrap(), // TODO: Make safer, don't just unwrap
            data: vec![0; 1], // TODO: Read from file (or something)
        }
    }

    pub fn generate_packet(&mut self) -> Packet {
        // Generate seed
        let seed = self.rng.next_u64();
        // Use seed to get degree for source
        // Encode data from degree source blocks
        // Build packet
        Packet {
            header: Header {
                block_size: BLOCK_SIZE,
                source_size: self.data.len() as u64,
            },
            block: RawEncodedBlock {
                data: vec![0; BLOCK_SIZE as usize],
                seed: seed,

            }
        }
    }
}
