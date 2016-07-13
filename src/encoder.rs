//! Luby Transform encoder
extern crate rand;

use rand::{Rng, StdRng};
use std::fs::{File, Metadata};
use std::io;
use std::io::Read;
use std::path::Path;

use Header;
use Packet;
use RawEncodedBlock;

const BLOCK_SIZE: u64 = 1024; // Bytes

struct Source {
    blocks: Vec<Vec<u8>>,
    metadata: Metadata,
    rng: StdRng,
}

impl Source {
    fn new_from_file<P: AsRef<Path>>(path: P) -> io::Result<Source> {
        let mut file = try!(File::open(path));
        let metadata = try!(file.metadata());

        let block_count: usize = (metadata.len() / BLOCK_SIZE) as usize;
        let mut blocks: Vec<Vec<u8>> = Vec::new();

        for i in 0..block_count {
            blocks.push(Vec::new());

            if i < blocks.len() - 1 {
                file.by_ref().take(BLOCK_SIZE).read_to_end(&mut blocks[i]);
            } else {
                file.read_to_end(&mut blocks[i]);
            }
        }

        Ok(Source {
            blocks: blocks,
            metadata: metadata,
            rng: StdRng::new().unwrap(), // TODO: Make safer, don't just unwrap
        })
    }

    pub fn generate_packet(&mut self) -> Packet {
        // Generate seed
        let seed = self.rng.next_u64();
        // TODO: Use seed to get degree for source
        // TODO: Encode data from degree source blocks
        // TODO: Build packet
        Packet {
            header: Header {
                block_size: BLOCK_SIZE,
                source_size: self.metadata.len(),
            },
            block: RawEncodedBlock {
                data: vec![0; BLOCK_SIZE as usize],
                seed: seed,

            }
        }
    }
}
