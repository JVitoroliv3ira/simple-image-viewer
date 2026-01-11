#[derive(Debug, Clone)]
pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl ImageAsset {
    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Result<Self, String> {
        let bytes_expected = (width as usize) * (height as usize) * 4;
        if rgba.len() != bytes_expected {
            return Err(format!(
                "Buffer RGBA inválido: esperado {bytes_expected} bytes, recebeu {}",
                rgba.len()
            ));
        }

        Ok(Self {
            width,
            height,
            rgba,
        })
    }
}
