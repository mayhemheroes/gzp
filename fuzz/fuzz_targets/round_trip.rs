#![no_main]

use std::{
    io::{Cursor, Read, Write},
    sync::{Arc, Mutex},
};

use arbitrary::Arbitrary;
use gzp::{
    deflate::{Bgzf, Mgzip},
    par::{compress::ParCompressBuilder, decompress::ParDecompressBuilder},
    BlockFormatSpec, GzpError, ZWriter,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Format {
    Bgzf,
    Mgzip,
}

/// A wrapper so that the data can be shared across threads
struct SendableData<T> {
    arc: Arc<Mutex<T>>,
}

impl<T: Read> Read for SendableData<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.arc.lock().unwrap().read(buf)
    }
}

impl<T: Write> Write for SendableData<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.arc.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.arc.lock().unwrap().flush()
    }
}

fuzz_target!(|input: (Format, Vec<u8>)| {
    let (format, data) = input;

    let _ = match format {
        Format::Bgzf => test::<Bgzf>(data),
        Format::Mgzip => test::<Mgzip>(data),
    };
});

fn test<F: BlockFormatSpec>(data: Vec<u8>) -> Result<(), GzpError> {
    let compressed = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
    let mut compressor = ParCompressBuilder::<F>::new().from_writer(SendableData {
        arc: compressed.clone(),
    });
    compressor.write_all(&data)?;
    compressor.finish()?;

    let format = F::new();
    let mut decompressor = format.create_decompressor();

    let decompressed = format.decode_block(
        &mut decompressor,
        compressed.lock().unwrap().get_ref().as_slice(),
        data.len(),
    )?;

    // let mut decompressed = vec![0_u8; data.len()];
    // let mut decompressor = ParDecompressBuilder::<F>::new().from_reader(SendableData {
    //     arc: compressed.clone(),
    // });
    // let bytes_read = decompressor.read_to_end(&mut decompressed)?;
    // decompressor.finish()?;

    if data != decompressed {
        println!("Original: {data:?}");
        println!("After: {decompressed:?}");
        println!("Compressed: {:?}", compressed.lock().unwrap().get_ref());
        panic!("Compression and decompression changed data");
    }

    Ok(())
}
