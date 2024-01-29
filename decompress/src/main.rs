use flate2::read::GzDecoder;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use tar::Archive;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("path/to/archive.tar.gz")?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    println!("Extracted:");

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_owned();

        // Extract file or directory
        entry.unpack(&path)?;

        println!("> {}", path.display());
    }

    Ok(())
}
