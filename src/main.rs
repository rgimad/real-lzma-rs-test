use std::env;
use std::path::Path;
use std::io::{BufReader, Cursor, Write};
use std::time::Duration;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::error::Error;

fn push_u32_le(vec: &mut Vec<u8>, value: u32) {
    vec.push((value & 0xFF) as u8);         // Least significant byte
    vec.push(((value >> 8) & 0xFF) as u8);
    vec.push(((value >> 16) & 0xFF) as u8);
    vec.push(((value >> 24) & 0xFF) as u8); // Most significant byte
}

fn push_u64_le(vec: &mut Vec<u8>, value: u64) {
    vec.push((value & 0xFF) as u8);         // Least significant byte
    vec.push(((value >> 8) & 0xFF) as u8);
    vec.push(((value >> 16) & 0xFF) as u8);
    vec.push(((value >> 24) & 0xFF) as u8);
    vec.push(((value >> 32) & 0xFF) as u8);
    vec.push(((value >> 40) & 0xFF) as u8);
    vec.push(((value >> 48) & 0xFF) as u8);
    vec.push(((value >> 56) & 0xFF) as u8);
}

fn push_u32_be(vec: &mut Vec<u8>, value: u32) {
    vec.push(((value >> 24) & 0xFF) as u8);  // Least significant byte
    vec.push(((value >> 16) & 0xFF) as u8);
    vec.push(((value >> 8) & 0xFF) as u8);
    vec.push((value & 0xFF) as u8); // Most significant byte
}

fn push_u32_le_alt(vec: &mut Vec<u8>, value: u32) {
    vec.extend_from_slice(&value.to_le_bytes());
}

fn main() -> Result<(), Box<dyn Error>> {

    let file_path = "nix.skn";

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut file_data = &buffer[..];
    let mut cursor1 = Cursor::new(file_data);

    let magic = cursor1.read_u32::<LittleEndian>()?;
    let unpack_size = cursor1.read_u32::<LittleEndian>()?;
    let flags = cursor1.read_u32::<LittleEndian>()?;

    println!("unpack_size = {:X?}", unpack_size);

    assert!(magic == 0x4B43504B);
    assert!(flags == 1);

    let lzma_props: u8 = (2*5+0)*9+3;
    let mut lzma_dict_size = 0x10000u32;
    let lzma_unpack_size = unpack_size as u64;

    while lzma_dict_size < unpack_size {
        lzma_dict_size *= 2;
    }

    println!("lzma_dict_size = {:X?}", lzma_dict_size);

    let mut lzma_buf: Vec<u8> = Vec::new();
    lzma_buf.push(lzma_props);
    // push_u32_le(&mut lzma_buf, 0x800000);
    push_u32_le(&mut lzma_buf, lzma_dict_size);

    push_u64_le(&mut lzma_buf, lzma_unpack_size);
    lzma_buf.push(0 as u8);  // TODO: WHY?

    lzma_buf.extend_from_slice(&file_data[4*3..]);

    let data_offset = 14;
    // TODO: WHY? ?
    lzma_buf.swap(data_offset+0, data_offset+3);
    lzma_buf.swap(data_offset+1, data_offset+2);

    println!("lzma_buf = {:X?} ", &lzma_buf);

    let mut unpacked_data: Vec<u8> = Vec::new();
    // println!("fdfdf = {}", unpacked_data.len());
       
    //let mut lzma_buf_reader = BufReader::new(lzma_buf.as_slice());
    //lzma_rs::lzma_decompress(&mut lzma_buf_reader, &mut unpacked_data).unwrap();
    lzma_rs::lzma_decompress(&mut lzma_buf.as_slice(), &mut unpacked_data).unwrap();
    
    let mut file = File::create("nix_unpacked.skn")?; // Create the file (or overwrite if it exists)
    file.write_all(&unpacked_data)?;

    Ok(())
}

// https://github.com/KolibriOS/kolibrios/blob/main/programs/system/skincfg/trunk/unpacker.inc#L87
