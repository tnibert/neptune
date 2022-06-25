extern crate find_folder;

pub fn convert_renderable(img: &im::RgbaImage, texture_context: &mut piston_window::G2dTextureContext) -> piston_window::G2dTexture {
    return piston_window::Texture::from_image(
            texture_context,
            &img,
            &piston_window::TextureSettings::new()
        ).unwrap();
}

pub fn load_image_asset_buffer(fname: &str) -> im::RgbaImage {
    let assets = find_folder::Search::Parents(3).for_folder("assets").unwrap();
    let img_path = assets.join(fname);
    return im::open(img_path).unwrap().into_rgba8();
    //let img = im::imageops::crop(&mut img, 0, 0, 30, 40).to_image();
    //im::imageops::overlay(&mut img, &on_top, 128, 128);
}