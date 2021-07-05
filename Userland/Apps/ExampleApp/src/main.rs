#![no_std]

use mx_std::{
    convert::From,
    handle::Result,
    obj::{
        impls::file::File,
        UserCreatableObject
    },
    path::Path
};

fn main() -> Result<()> {
    let file_path = Path::from("/Users/Marco/Docs/example.txt");
    let f = File::creat().for_read().for_write().apply_for(&file_path)?;

    let mut _read_buf = [0u8; 512];
    f.read(&mut _read_buf)?;

    Ok(())
}
