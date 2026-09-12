use std::fs;
use std::io;
use std::path::Path;

pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    fs::read(path)
}
pub fn write(path: impl AsRef<Path>, data: &[u8]) -> io::Result<()> {
    fs::write(path, data)
}
