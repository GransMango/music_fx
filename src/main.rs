mod wav_file_handling;

fn main() {
    let filename = "test.wav";
    if let Err(e) = wav_file_handling::read_wav_file(filename) {
        println!("Failed to read the file, error code {}", e);
    } else {
        println!("File read!");
    }
}