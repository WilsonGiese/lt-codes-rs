use rand::{Rng, SeedableRng, StdRng, Seed};

struct Packet {
    header: Header,
    block: RawEncodedBlock,
}

struct Header {
    // Size of block in bytes
    block_size: u64,
    // Size of source in bytes (Final block will be padded?)
    source_size: u64,
}

///
struct RawEncodedBlock {
    // Encoded data
    data: Vec<u8>,
    // Seed for PRNG
    seed: Seed,
}

struct EncodedBlock {
    // Encoded data (until len(sources) == 1)
    data: Vec<u8>,
    // Set of source block indicies which makeup the encoded data
    sources: Vec<u64>,
}

struct SourceBlock {
    // Decoded data
    data: Vec<u8>,
    // Pointers to encoded blocks which encode this source block
    encoded_blocks: Vec<&EncodedBlock>,
}

struct Source {
    // Entirety of decoded source data
    data: Vec<SourceBlock>, 
}
