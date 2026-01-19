use snap::raw::{Encoder, Decoder};

pub fn compress(input: &[u8]) -> Result<Vec<u8>, snap::Error> {
    let mut encoder = Encoder::new();
    encoder.compress_vec(input)
}

pub fn decompress(input: &[u8]) -> Result<Vec<u8>, snap::Error> {
    let mut decoder = Decoder::new();
    decoder.decompress_vec(input)
}
