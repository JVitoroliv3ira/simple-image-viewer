use image::GenericImageView;

use crate::domain::ImageAsset;
use std::path::Path;

pub fn load_image(path: &Path) -> Result<ImageAsset, String> {
    let image = image::open(path)
        .map_err(|e| format!("Erro ao abrir a imagem '{}' : {e}", path.display()))?;

    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw();

    ImageAsset::new(width, height, rgba)
}
