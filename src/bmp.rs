use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
pub fn write_bmp(filename: &str, width: usize, height: usize, rgb_buffer: &[u8]) -> std::io::Result<()> {
    let row_padded = (width * 3 + 3) & !3;
    let data_size = row_padded * height;
    let file_size = 54 + data_size;

    let mut file = File::create(filename)?;

    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&0u16.to_le_bytes())?;
    file.write_all(&0u16.to_le_bytes())?;
    file.write_all(&54u32.to_le_bytes())?;

    file.write_all(&40u32.to_le_bytes())?;
    file.write_all(&(width as i32).to_le_bytes())?;
    file.write_all(&(height as i32).to_le_bytes())?;
    file.write_all(&1u16.to_le_bytes())?;
    file.write_all(&24u16.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&(data_size as u32).to_le_bytes())?;
    file.write_all(&2835i32.to_le_bytes())?;
    file.write_all(&2835i32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;
    file.write_all(&0u32.to_le_bytes())?;

    let padding = vec![0u8; row_padded - width * 3];
    for y in (0..height).rev() {
        for x in 0..width {
            let idx = (y * width + x) * 3;
            let r = rgb_buffer[idx];
            let g = rgb_buffer[idx + 1];
            let b = rgb_buffer[idx + 2];
            file.write_all(&[b, g, r])?;
        }
        file.write_all(&padding)?;
    }

    Ok(())
}