use std::io;
pub fn get_password() -> io::Result<Vec<u8>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().as_bytes().to_vec())
}
