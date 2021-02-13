use image::{GenericImageView, ImageBuffer, RgbImage, EncodableLayout};
use std::collections::HashMap;

fn main() {
    let pixels = image::open("isometric.png").unwrap();
    let bytes = pixels.as_rgb8().unwrap().as_raw();

    let cellwidth = pixels.width() / 32;
    let cellheight = pixels.height() / 24;

    let mut out_images: HashMap<Vec<u8>, (String, RgbImage)> = HashMap::new();

    std::fs::remove_dir_all("out/");
    std::fs::create_dir("out/").unwrap();

    for cellx in 0..cellwidth {
        for celly in 0..cellheight {

            let start_x = cellx * 32;
            let start_y = celly * 24;

            let mut image: RgbImage = ImageBuffer::new(32, 24);
            let mut image_bytes = image.as_mut();

            let mut hash = Vec::with_capacity(32 * 24 * 3);

            for x in start_x..(start_x + 32) {
                for y in start_y..(start_y + 24) {
                    let index = (y * pixels.width() + x) as usize;
                    let smaller_index = ((y - start_y) * 32 + (x - start_x)) as usize;

                    image_bytes[smaller_index * 3] = bytes[index * 3];
                    image_bytes[smaller_index * 3 + 1] = bytes[index * 3 + 1];
                    image_bytes[smaller_index * 3 + 2] = bytes[index * 3 + 2];

                    hash.push(bytes[index * 3]);
                    hash.push(bytes[index * 3 + 1]);
                    hash.push(bytes[index * 3 + 2]);
                }
            }

            out_images.insert(hash, ("x".to_string() + cellx.to_string().as_str() + "_y" + celly.to_string().as_str(), image));
        }
    }

    for (hash, (name, image)) in out_images {
        println!("image {:?}", name);
        image::save_buffer("out/".to_string() + name.as_str() + ".png", image.as_bytes(), 32, 24, image::ColorType::Rgb8).unwrap();
    }
}
