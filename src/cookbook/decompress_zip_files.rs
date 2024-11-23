use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub fn decompress_tar_xz(path: &str) -> Result<(), std::io::Error> {
    let tar_xz = File::open(path)?;
    let tar = GzDecoder::new(tar_xz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

use flate2::{write::GzEncoder, Compression};
use tar::Builder;

pub fn compress_into_tar_gz(path: &str, gz_file_name: &str) -> Result<(), std::io::Error> {
    let tar_gz = File::create(gz_file_name)?;
    let encoder = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(encoder);
    tar.append_path(path)?;

    Ok(())
}
