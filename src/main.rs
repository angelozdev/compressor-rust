use flate2::{write::GzEncoder, Compression};
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::time;

use compressor::config::Config;

const BUFFER_SIZE: usize = 1024;

fn main() {
    let config = Config::new();
    let now = time::Instant::now();

    println!("Compressing {}...", &config.source);

    match compress_file(&config.source) {
        Ok(_) => println!("{:?}", now.elapsed()),
        Err(e) => panic!("No se pudo comprimir el archivo: {}", e),
    }
}

fn read_in_chunks(file: &File, mut callback: impl FnMut(&[u8])) {
    let mut reader = BufReader::new(file);
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(len) => {
                let chunk = &buffer[..len];
                callback(chunk);
            }
            Err(e) => {
                eprintln!("Error al leer un bloque de datos: {}", e);
                break;
            }
        }
    }
}

fn compress_file(path: &str) -> Result<(), Error> {
    let file = File::open(path)?;

    let new_file_name = format!("{}.gz", path);
    let new_file = File::create(&new_file_name)?;
    let writer = BufWriter::new(new_file);
    let mut encoder = GzEncoder::new(writer, Compression::default());

    read_in_chunks(&file, |chunk| {
        encoder.write_all(chunk).expect("Error al escribir");
    });

    encoder.finish()?;

    Ok(())
}
