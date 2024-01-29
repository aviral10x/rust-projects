use std::fs::File;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

fn main() -> Result<(), std::io::Error> {
    // Create a file for the compressed output
    let gz = File::create("archive.tar.gz")?;

    // Create a Gzip encoder
    let encoder = GzEncoder::new(gz, Compression::default());

    // Create a Tar builder with the encoder
    let mut tar = Builder::new(encoder);

    // Add all files in the current directory to the archive
    tar.append_dir_all(".", "current_backup")?;

    // Finish the tar archive
    tar.into_inner()?.finish()?;

    Ok(())
}
