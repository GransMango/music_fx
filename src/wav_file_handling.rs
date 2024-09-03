use std::io::{self, Read, Seek, SeekFrom};
use std::fs::File;

pub fn read_wav_file(filename : &str) -> std::io::Result<()> {
    let file = File::open(filename)?;

    let sample_rate = read_wav_sample_rate(file)?;

    println!("Read sample rate {} from file", sample_rate);


    Ok(())
}

fn read_wav_sample_rate(mut file : File) -> io::Result<u32> {
    let mut buffer = [0; 4];

    // Sample rate in a WAV file is in 24th byte position
    file.seek(SeekFrom::Start(24))?;

    file.read_exact(&mut buffer)?;

    // Stored as a little endian unsigned integer.
    let sample_rate = u32::from_le_bytes(buffer);
    Ok(sample_rate)
}

fn wav_sample_chopping(mut file : File) {

}