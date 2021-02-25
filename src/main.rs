use image::{GenericImageView, ImageBuffer, RgbImage, EncodableLayout, RgbaImage};
use std::collections::HashMap;
use std::fs;

fn main() {
    // cut_image();

    // generate_grid();

    generate_level();
}

fn generate_level() {
    let pixels = image::open("isometric.png").unwrap();
    let bytes = pixels.as_rgb8().unwrap().as_raw();

    let input_cell_width = 32;
    let input_cell_height = 16;

    let output_cell_width = 32;
    let output_cell_height = 24;

    let x_count = pixels.width() / input_cell_width;
    let y_count = pixels.height() / input_cell_height;

    let mut indices: Vec<i32> = vec![0; (x_count * y_count) as usize];

    let mut out_tiles: HashMap<Vec<u8>, i32> = HashMap::new();

    std::fs::remove_dir_all("out/");
    std::fs::create_dir("out/").unwrap();

    for celly in 0..y_count {
        for cellx in 0..x_count {
            let start_x = cellx * input_cell_width;
            let start_y = celly * input_cell_height;

            let image_index = (celly * x_count + cellx) as usize;

            let mut hash = Vec::with_capacity((input_cell_width * input_cell_height * 3) as usize);

            for y in start_y..(start_y + input_cell_height) {
                for x in start_x..(start_x + input_cell_width) {
                    let index = (y * pixels.width() + x) as usize;

                    hash.push(bytes[index * 3]);
                    hash.push(bytes[index * 3 + 1]);
                    hash.push(bytes[index * 3 + 2]);
                }
            }

            if !out_tiles.contains_key(&hash) {
                let tileset_index = out_tiles.len() as i32;

                out_tiles.insert(hash.clone(), tileset_index);
            }

            let tileset_index = *out_tiles.get(&hash).unwrap();
            indices[image_index] = tileset_index;
        }
    }

    let mut min_area = 0xffffff;
    let mut best_x_count = 0;
    let mut best_y_count = 0;
    for out_x_count in 1..out_tiles.len() {
        let out_y_count = (out_tiles.len() as f32 / out_x_count as f32).ceil() as i32;

        let out_width = out_x_count as i32 * (output_cell_width + 4);
        let out_height = out_y_count as i32 * (output_cell_height + 4);
        if out_width > 1024 || out_height > 1024 {
            continue;
        }

        let area = out_width * out_height;
        if area < min_area {
            min_area = area;
            best_x_count = out_x_count as i32;
            best_y_count = out_y_count;
        }
    }

    let out_image_width = (best_x_count * (output_cell_width + 4)) as i32;
    let out_image_height = (best_y_count * (output_cell_height + 4)) as i32;

    let mut out_image: RgbImage = ImageBuffer::new(out_image_width as u32, out_image_height as u32);
    let mut out_bytes = out_image.as_mut();
    let tileset_x_count = best_x_count;

    for (bytes, index) in &out_tiles {
        let index_y = index / tileset_x_count as i32;
        let index_x = index - index_y * tileset_x_count;

        let start_x = index_x * (output_cell_width + 4) as i32 + 2;
        let start_y = index_y * (output_cell_height + 4) as i32 + 2;

        for x_add in 0..output_cell_width {
            for y_add in 0..output_cell_height {
                let scaled_x_add = (x_add as f32 * (input_cell_width as f32 / output_cell_width as f32)) as i32;
                let scaled_y_add = (y_add as f32 * (input_cell_height as f32 / output_cell_height as f32)) as i32;

                let byte_index = (scaled_y_add * input_cell_width as i32 + scaled_x_add) as usize;

                let r = bytes[byte_index * 3];
                let g = bytes[byte_index * 3 + 1];
                let b = bytes[byte_index * 3 + 2];

                let real_x = start_x + x_add as i32;
                let real_y = start_y + y_add as i32;
                // let real_index = (y_add * out_image_width + x_add) as usize;
                let real_index = (real_y * out_image_width as i32 + real_x) as usize;

                out_bytes[real_index * 3] = r;
                out_bytes[real_index * 3 + 1] = g;
                out_bytes[real_index * 3 + 2] = b;
            }
        }
    }

    image::save_buffer("out/".to_string() + "ctftileset.png", out_image.as_bytes(), out_image_width as u32, out_image_height as u32, image::ColorType::Rgb8).unwrap();

    let tileset_body = r#"
        { "columns":COLUMNS,
          "image":"ctftileset.png",
          "imageheight":IMAGEHEIGHT,
          "imagewidth":IMAGEIWDTH,
          "margin":2,
          "name":"ctftileset",
          "spacing":4,
          "tilecount":TILECOUNT,
          "tiledversion":"1.3.1",
          "tileheight":TILEHEIGHT,
          "tilewidth":TILEWIDTH,
          "type":"tileset",
          "version":1.2
        }
    "#;
    let tileset_body = tileset_body.replace("COLUMNS", tileset_x_count.to_string().as_str());
    let tileset_body = tileset_body.replace("IMAGEHEIGHT", out_image_height.to_string().as_str());
    let tileset_body = tileset_body.replace("IMAGEIWDTH", out_image_width.to_string().as_str());
    let tileset_body = tileset_body.replace("TILECOUNT", out_tiles.len().to_string().as_str());
    let tileset_body = tileset_body.replace("TILEHEIGHT", output_cell_height.to_string().as_str());
    let tileset_body = tileset_body.replace("TILEWIDTH", output_cell_width.to_string().as_str());

    fs::write("out/ctftileset.json", tileset_body);

    let map_body = r#"
        { "compressionlevel":-1,
          "height":ROWCOUNT,
          "infinite":false,
          "layers":[
                 {
                  "data":TILEDATA,
                  "height":ROWCOUNT,
                  "id":1,
                  "name":"Tile Layer 1",
                  "opacity":1,
                  "type":"tilelayer",
                  "visible":true,
                  "width":COLUMNCOUNT,
                  "x":0,
                  "y":0
                 }],
          "nextlayerid":2,
          "nextobjectid":1,
          "orientation":"orthogonal",
          "renderorder":"right-down",
          "tiledversion":"1.3.1",
          "tileheight":TILEHEIGHT,
          "tilesets":[
                 {
                  "firstgid":1,
                  "source":"ctftileset.json"
                 }],
          "tilewidth":TILEWIDTH,
          "type":"map",
          "version":1.2,
          "width":COLUMNCOUNT
        }
    "#;
    let map_body = map_body.replace("ROWCOUNT", y_count.to_string().as_str());
    let map_body = map_body.replace("COLUMNCOUNT", x_count.to_string().as_str());
    let map_body = map_body.replace("TILEHEIGHT", output_cell_height.to_string().as_str());
    let map_body = map_body.replace("TILEWIDTH", output_cell_width.to_string().as_str());
    let indices_string: Vec<String> = indices.iter().map(|v| (v + 1).to_string()).collect();
    let indices_string: String = indices_string.join(",");
    println!("{:?} {:?} {:?}", x_count, y_count, indices.len());
    let map_body = map_body.replace("TILEDATA", ("[".to_string() + indices_string.as_str() + "]").as_str());

    fs::write("out/ctf.json", map_body);
}

fn generate_grid() {
    let image_width = 2240;
    let image_height = 984;

    let cell_width = 16;
    let cell_height = 8;

    let count_x = 2;
    let count_y = 2;

    let tile_width = cell_width * count_x;
    let tile_height = cell_height * count_y;

    let mut image: RgbaImage = ImageBuffer::new(image_width, image_height);
    let mut image_bytes = image.as_mut();

    std::fs::remove_dir_all("out/");
    std::fs::create_dir("out/").unwrap();

    for y in 0..image_height {
        for x in 0..image_width {
            let index = (y * image_width + x) as usize;

            let cell_x = x / tile_width;
            let cell_y = y / tile_height;

            let odd_x = cell_x % 2;
            let odd_y = cell_y % 2;

            if odd_x == odd_y {
                image_bytes[index * 4] = 0;
                image_bytes[index * 4 + 1] = 0;
                image_bytes[index * 4 + 2] = 0;
            } else {
                image_bytes[index * 4] = 70;
                image_bytes[index * 4 + 1] = 70;
                image_bytes[index * 4 + 2] = 70;
            }

            image_bytes[index * 4 + 3] = 127;
        }
    }

    image::save_buffer("out/".to_string() + "grid.png", image.as_bytes(), image_width, image_height, image::ColorType::Rgba8).unwrap();
}

fn cut_image() {
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
