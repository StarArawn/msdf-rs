use std::fs::File;
use std::io::BufWriter;

use msdf::bitmap::FloatRGBBmp;
use msdf::ttf_parser::ShapeBuilder;
use msdf::vector::Vector2;

fn main() {
    let size = 64;
    let mut output = FloatRGBBmp::new(size, size);

    let font_data = include_bytes!("../resources/Roboto-Regular.ttf");

    let face = ttf_parser::Face::from_slice(font_data, 0).unwrap();

    let c = 'U';
    if let Some(glyph) = face.glyph_index(c) {
        let mut builder = ShapeBuilder::default();
        builder.pixel_scale = size as f64 / face.units_per_em() as f64;
        let result = face.outline_glyph(glyph, &mut builder);
        dbg!(result);

        let mut shape = builder.build();

        msdf::edge_coloring::simple(&mut shape, 4.0, 0);

        let range = 4.0 / 1.0;

        msdf::gen::generate_msdf(
            &mut output,
            &shape,
            range,
            Vector2::new(1.0, 1.0),
            Vector2::new(0.0, 0.0),
            1.0000001,
        );
    }

    let file = File::create("./test.png").unwrap();
    let ref mut w = BufWriter::new(file);

    let pixels: Vec<u8> = output
        .buffer
        .iter()
        .map(|pixel| {
            vec![
                (pixel.r * 255.0) as u8,
                (pixel.g * 255.0) as u8,
                (pixel.b * 255.0) as u8,
                255u8,
            ]
        })
        .flatten()
        .collect();

    let mut encoder = png::Encoder::new(w, size as u32, size as u32);
    encoder.set_color(png::ColorType::Rgba);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();
}
