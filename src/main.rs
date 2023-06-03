use cosmic_text::*;
#[cfg(feature = "render")]
use image::{ImageBuffer, Rgba, RgbaImage};

fn main() {
    const TEXT: &str = "(6)  SomewhatBoringDisplayTransform";
    const FONT_SIZE: f32 = 18.0;
    const LINE_HEIGHT: f32 = FONT_SIZE * 1.2;
    const UNBOUNDED: f32 = f32::MAX;

    let mut font_system =
        FontSystem::new_with_locale_and_db("en-US".to_string(), fontdb::Database::new());
    font_system
        .db_mut()
        .load_font_file("fonts/FiraMono-Medium.ttf")
        .unwrap();

    let metrics = Metrics::new(FONT_SIZE, LINE_HEIGHT);

    let mut buffer1 = Buffer::new(&mut font_system, metrics.clone());
    let mut buffer2 = Buffer::new(&mut font_system, metrics);

    let attrs = Attrs::new()
        .family(Family::Name("FiraMono"))
        .weight(Weight::MEDIUM);

    buffer1.set_text(&mut font_system, TEXT, attrs);
    buffer2.set_text(&mut font_system, TEXT, attrs);

    buffer1.set_size(&mut font_system, UNBOUNDED, UNBOUNDED);
    let widths1 = buffer_run_widths(&buffer1);

    assert_eq!(widths1.len(), 1);

    assert_eq!(widths1, [377.9999]);

    // use the width of `buffer1` as the width constraint for `buffer2`
    let width1 = widths1[0];

    // `width1.ceil()` still produces the same behaviour
    buffer2.set_size(&mut font_system, width1, UNBOUNDED);
    let widths2 = buffer_run_widths(&buffer2);

    assert_ne!(&widths1, &widths2);

    assert_eq!(widths2.len(), 2);

    assert_eq!(widths2, [43.2, 323.99997]);

    #[cfg(feature = "render")]
    {
        let mut swash_cache = SwashCache::new();

        const IMAGE_WIDTH: u32 = 512;
        const IMAGE_HEIGHT: u32 = 256;

        let mut img: RgbaImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

        buffer1.draw(
            &mut font_system,
            &mut swash_cache,
            Color::rgb(0x00, 0xFF, 0x00),
            |x, y, w, h, color| {
                if color.a() == 0
                    || x < 0
                    || x > IMAGE_WIDTH as i32
                    || y < 0
                    || y > IMAGE_HEIGHT as i32
                    || w != 1
                    || h != 1
                {
                    // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                    return;
                }
                img.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([color.r(), color.g(), color.b(), color.a()]),
                );
            },
        );

        buffer2.draw(
            &mut font_system,
            &mut swash_cache,
            Color::rgb(0xFF, 0x00, 0x00),
            |x, y, w, h, color| {
                if color.a() == 0
                    || x < 0
                    || x > IMAGE_WIDTH as i32
                    || y < 0
                    || y > IMAGE_HEIGHT as i32
                    || w != 1
                    || h != 1
                {
                    // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                    return;
                }
                img.put_pixel(
                    x as u32,
                    y as u32 + IMAGE_HEIGHT / 2,
                    Rgba([color.r(), color.g(), color.b(), color.a()]),
                );
            },
        );
        img.save("out.png").unwrap();
    }
}

fn buffer_run_widths(buffer: &Buffer) -> Vec<f32> {
    buffer.layout_runs().map(|run| run.line_w).collect()
}
