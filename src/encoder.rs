//! Luby Transform encoder
extern crate rand;

use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use Header;
use Packet;
use RawEncodedBlock;

struct Source {
    size: u64,            // Size of the source in bytes
    block_size: u64,
    blocks: Vec<Vec<u8>>,
    rng: StdRng,          // Standard PRNG used for seed generation (TODO: det if should each souce have its own?)
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
            size: size,
            block_size: block_size,
            blocks: blocks,
            rng: StdRng::new().unwrap(), // TODO: Make safer, don't just unwrap
        })
    }

    fn new_from_file<P: AsRef<Path>>(path: P, block_size: u64) -> io::Result<Source> {
        Source::new(try!(File::open(path)), block_size)
    }

    pub fn generate_packet(&mut self) -> Packet {
        // Use a HashSet to track blocks we've already encoded (so we only encode a block once)
        let mut used_blocks: HashSet<usize> = HashSet::new();

        // Generate seed
        let seed: &[usize] = &[self.rng.next_u64() as usize];
        let mut seedableRng: StdRng = SeedableRng::from_seed(seed);

        // TODO: Use seed to generate an actual degree from a real distribution
        let degree = seedableRng.next_u64() as usize % self.blocks.len();

        // Encode degree number of blocks into encoded_blocks
        let mut encoded_block = vec![0; self.block_size as usize];

        let mut encoded_count = 0;
        while encoded_count < degree {
            let blocks_i = seedableRng.next_u64() as usize % self.blocks.len();

            if !used_blocks.contains(&blocks_i) {
                // Encode by XORing each byte in the current encoded block with a new block
                for i in 0..self.block_size as usize {
                    encoded_block[i] ^= self.blocks[blocks_i][i];
                }
                used_blocks.insert(blocks_i);
                encoded_count += 1;
            }
        }

        Packet {
            header: Header {
                block_size: self.block_size,
                source_size: self.size,
            },
            block: RawEncodedBlock {
                data: encoded_block,
                seed: seed[0] as u64,
            }
        }
    }
}

#[test]
fn test_seedable_rng() {
    let mut rng = StdRng::new().unwrap();

    let seed: &[usize] = &[rng.next_u64() as usize];
    let mut seedableRng1: StdRng = SeedableRng::from_seed(seed);
    let mut seedableRng2: StdRng = SeedableRng::from_seed(seed);

    for _ in 0..9999 {
        assert!(seedableRng1.next_u64() == seedableRng2.next_u64());
    }
}
