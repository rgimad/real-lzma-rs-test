use std::env;
use std::path::Path;
use std::io::{BufReader, Cursor};
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


    let mut comp: Vec<u8> = Vec::new();
    lzma_rs::lzma_compress(&mut std::io::BufReader::new("textazazzazaa".as_bytes()), &mut comp).unwrap();
    println!("comp = {:X?} ", &comp);




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
    let lzma_unpack_size = unpack_size;

    while lzma_dict_size < unpack_size {
        lzma_dict_size *= 2;
    }

    println!("lzma_dict_size = {:X?}", lzma_dict_size);

    let mut lzma_buf: Vec<u8> = Vec::new();
    lzma_buf.push(lzma_props);
    // push_u32_le(&mut lzma_buf, 0x800000);
    push_u32_le(&mut lzma_buf, lzma_dict_size);

    push_u32_le(&mut lzma_buf, 0xFFFFFFFF);
    push_u32_le(&mut lzma_buf, 0xFFFFFFFF);

    // push_u32_le(&mut lzma_buf, lzma_unpack_size);
    // push_u32_le(&mut lzma_buf, 0);

    // lzma_buf.push(0);

    // let tmp = cursor1.read_u32::<LittleEndian>()?;
    // push_u32_be(&mut lzma_buf, tmp);

    lzma_buf.extend_from_slice(&file_data[4*3..]);

    println!("lzma_buf = {:X?} ", &lzma_buf);

    let mut unpacked_data: Vec<u8> = Vec::new();
    // println!("fdfdf = {}", unpacked_data.len());
       
    //let mut lzma_buf_reader = BufReader::new(lzma_buf.as_slice());
    //lzma_rs::lzma_decompress(&mut lzma_buf_reader, &mut unpacked_data).unwrap();
    lzma_rs::lzma_decompress(&mut lzma_buf.as_slice(), &mut unpacked_data).unwrap();
        

    // let mut f = std::io::BufReader::new(std::fs::File::open(filename).unwrap());

    // let mut decomp: Vec<u8> = Vec::new(); // can be anything that implements "std::io::Write"
    // lzma_rs::lzma_decompress(&mut f, &mut decomp).unwrap();

    // Decompressed content is now in "decomp"

    Ok(())
}

// https://github.com/KolibriOS/kolibrios/blob/main/programs/system/skincfg/trunk/unpacker.inc#L87
