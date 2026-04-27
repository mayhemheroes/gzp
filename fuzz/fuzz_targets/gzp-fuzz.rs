#![no_main]
use libfuzzer_sys::fuzz_target;
use gzp::{
    ZWriter,
    deflate::Mgzip,
    par::{compress::{ParCompress, ParCompressBuilder}}
};
use std::io::{Read, Write};

fuzz_target!(|data: &[u8]| {
    // let opt = data[0];
    // let new_data = &data[1..];
    // match opt{
    //     0=>{

    //     },
    //     1=>{

    //     },
    //     _=>()
    // }
    let mut dummy_output = vec![0 as u8; data.len() * 8];
    let mut writer: ParCompress<Mgzip> = ParCompressBuilder::new().from_writer(dummy_output);
    let chunk_size = 64;

    let mut buffer = Vec::with_capacity(chunk_size);
    let mut start = 0;
    let mut end = 0;
    loop {
        end = start + chunk_size;
        if end > data.len() {
            end = data.len();
        }
        let mut limit = &data[start..end];
        limit.read_to_end(&mut buffer).unwrap();
        if buffer.is_empty() {
            break;
        }
        writer.write_all(&buffer).unwrap();
        buffer.clear();
        start = end;
        if start >= data.len() {
            break;
        }
    }
    writer.finish().unwrap();
});
