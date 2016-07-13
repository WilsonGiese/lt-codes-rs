//! Luby Transform encoder
extern crate rand;

use rand::{Rng, SeedableRng, StdRng};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use Header;
use Packet;
use RawEncodedBlock;


struct Source {
    blocks: Vec<Vec<u8>>,
    block_size: u64,
    size: u64,
    rng: StdRng,
}

impl Source {
    fn new<R: Read>(mut source: R, block_size: u64) -> io::Result<Source> {
        let mut blocks: Vec<Vec<u8>> = Vec::new();
        let mut size: u64 = 0;

        loop {
            let mut vec = Vec::new();
            let read_bytes = try!(source.by_ref().take(block_size).read_to_end(&mut vec)) as u64;

            if read_bytes != 0 {
                blocks.push(vec);
                size += read_bytes;
            }

            if read_bytes != block_size {
                break;
            }
        }

        Ok(Source {
            blocks: blocks,
            block_size: block_size,
            size: size,
            rng: StdRng::new().unwrap(), // TODO: Make safer, don't just unwrap
        })
    }

    fn new_from_file<P: AsRef<Path>>(path: P, block_size: u64) -> io::Result<Source> {
        Source::new(try!(File::open(path)), block_size)
    }

    pub fn generate_packet(&mut self) -> Packet {
        // Generate seed
        let seed: &[_] = &[self.rng.next_u64() as usize];
        let seedableRng: StdRng = SeedableRng::from_seed(seed);

        // TODO: Use seed to get degree for source
        // TODO: Encode data from degree source blocks
        // TODO: Build packet
        Packet {
            header: Header {
                block_size: self.block_size,
                source_size: self.size,
            },
            block: RawEncodedBlock {
                data: vec![0; self.block_size as usize],
                seed: seed[0] as u64,
            }
        }
    }
}
