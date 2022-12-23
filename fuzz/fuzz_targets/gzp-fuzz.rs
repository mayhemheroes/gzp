#![no_main]

use arbitrary::Arbitrary;
use gzp::{
    deflate::{Bgzf, Gzip, Mgzip, RawDeflate, Zlib},
    par::compress::ParCompressBuilder,
    GzpError, ZWriter,
};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Format {
    Gzip,
    Zlib,
    Bgzf,
    Mgzip,
    RawDeflate,
}

fuzz_target!(|input: (Format, &[u8])| {
    let (format, data) = input;
    let dummy_output = vec![0_u8; data.len()];

    let _ = match format {
        Format::Gzip => test(
            ParCompressBuilder::<Gzip>::new().from_writer(dummy_output),
            data,
        ),
        Format::Zlib => test(
            ParCompressBuilder::<Zlib>::new().from_writer(dummy_output),
            data,
        ),
        Format::Bgzf => test(
            ParCompressBuilder::<Bgzf>::new().from_writer(dummy_output),
            data,
        ),
        Format::Mgzip => test(
            ParCompressBuilder::<Mgzip>::new().from_writer(dummy_output),
            data,
        ),
        Format::RawDeflate => test(
            ParCompressBuilder::<RawDeflate>::new().from_writer(dummy_output),
            data,
        ),
    };
});

const CHUNK_SIZE: usize = 64;

fn test<T: ZWriter>(mut writer: T, data: &[u8]) -> Result<(), GzpError> {
    for chunk in data.chunks(CHUNK_SIZE) {
        writer.write_all(chunk)?;
    }

    writer.finish()
}
