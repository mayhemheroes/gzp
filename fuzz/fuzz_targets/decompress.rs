#![no_main]

use std::{sync::{Arc, Mutex}, io::{Read, Cursor}};

use arbitrary::Arbitrary;
use gzp::{
    deflate::{Bgzf, Mgzip},
    par::decompress::{ParDecompressBuilder, ParDecompress},
    GzpError, BlockFormatSpec,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Format {
    Bgzf,
    Mgzip,
}

/// A wrapper so that the data can be shared across threads and read
struct SendableData {
    arc: Arc<Mutex<Cursor<Vec<u8>>>>
}

impl Read for SendableData {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.arc.lock().unwrap().read(buf)
    }
}

fuzz_target!(|input: (Format, Vec<u8>)| {
    let (format, data) = input;
    let mut dummy_output = vec![0_u8; data.len()];

    let arc = SendableData { arc: Arc::new(Mutex::new(Cursor::new(data))) };

    let _ = match format {
        Format::Bgzf => test(
            ParDecompressBuilder::<Bgzf>::new().from_reader(arc),
            &mut dummy_output,
        ),
        Format::Mgzip => test(
            ParDecompressBuilder::<Mgzip>::new().from_reader(arc),
            &mut dummy_output,
        ),
    };
});

fn test<F: BlockFormatSpec>(mut reader: ParDecompress<F>, output: &mut Vec<u8>) -> Result<(), GzpError> {
    let _ = reader.read_to_end(output);
    reader.finish()
}
