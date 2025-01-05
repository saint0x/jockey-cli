use std::io::prelude::*;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use flate2::write::GzEncoder;
use flate2::Compression;
use crate::error::{Result, JockeyError};
use std::sync::Arc;
use rayon::prelude::*;

const COMPRESSION_LEVEL: u32 = 6;  // Balanced compression level

pub struct CompressionPool {
    _private: (),  // Prevent direct construction
}

impl CompressionPool {
    pub fn new() -> Self {
        Self { _private: () }
    }

    pub fn compress_parallel<I>(&self, contents: I) -> Result<Vec<String>>
    where
        I: IntoParallelIterator<Item = Arc<String>>,
    {
        contents
            .into_par_iter()
            .map(|content| compress_content(&content))
            .collect()
    }
}

pub fn compress_content(content: &str) -> Result<String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(COMPRESSION_LEVEL));
    encoder.write_all(content.as_bytes()).map_err(|e| {
        JockeyError::Compression(format!("Failed to compress content: {}", e))
    })?;
    
    let compressed = encoder.finish().map_err(|e| {
        JockeyError::Compression(format!("Failed to finish compression: {}", e))
    })?;
    
    Ok(BASE64.encode(compressed))
}

pub fn decompress_content(compressed: &str) -> Result<String> {
    let decoded = BASE64.decode(compressed).map_err(|e| {
        JockeyError::Compression(format!("Failed to decode base64: {}", e))
    })?;
    
    let mut decoder = flate2::read::GzDecoder::new(&decoded[..]);
    let mut decompressed = String::new();
    
    decoder.read_to_string(&mut decompressed).map_err(|e| {
        JockeyError::Compression(format!("Failed to decompress content: {}", e))
    })?;
    
    Ok(decompressed)
} 