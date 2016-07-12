#[macro_use]
mod encoder;
mod decoder;

pub struct Packet {
    header: Header,
    block: RawEncodedBlock,
}

pub struct Header {
    // Size of block in bytes
    block_size: u64,
    // Size of source in bytes (Final block will be padded?)
    source_size: u64,
}

///
pub struct RawEncodedBlock {
    // Encoded data
    data: Vec<u8>,
    // Seed for PRNG
    seed: u64,
}

pub struct EncodedBlock {
    // Encoded data (until len(sources) == 1)
    data: Vec<u8>,
    // Set of source block indicies which makeup the encoded data
    sources: Vec<u64>,
}

pub struct SourceBlock<'a> {
    // Decoded data
    data: Vec<u8>,
    // Pointers to encoded blocks which encode this source block
    encoded_blocks: Vec<&'a EncodedBlock>,
}

pub struct Source<'a> {
    // Entirety of decoded source data
    data: Vec<&'a SourceBlock<'a>>,
}
