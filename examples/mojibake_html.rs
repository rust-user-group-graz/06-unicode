use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut fd = File::create("pile_of_poo.html")?;
//  fd.write(b"<!DOCTYPE html>\n<head><title>\
//\xf0\x9f\x92\xa9</title>\n")?;
    fd.write(b"<!DOCTYPE html>\n<head><title>\xda \
\xf0\x9f\x92\xa9</title>\n")?;
    Ok(())
}
