extern crate find_folder;

pub fn load_image_asset_buffer(fname: &str) -> im::RgbaImage {
    let assets = find_folder::Search::Parents(3).for_folder("assets").unwrap();
    let img_path = assets.join(fname);
    return im::open(img_path).unwrap().into_rgba8();
}