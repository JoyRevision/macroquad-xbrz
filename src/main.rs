use macroquad::prelude::*;

pub mod xbrz;

// TODO: Handle errors
pub async fn load_xbrz_texture(path: String, scale: usize) -> Texture2D {
    let mut image: Image = load_image(path.as_str()).await.unwrap();

    image.bytes = xbrz::scale(scale, &mut image.bytes, image.width, image.height);
    image.width *= scale as u16;
    image.height *= scale as u16;

    let texture = Texture2D::from_image(&image);
    return texture;
}

#[macroquad::main("xBRZ Example")]
async fn main() {
    let scale_factor = 3;

    let original = load_texture("res/hut.png").await.unwrap();
    original.set_filter(FilterMode::Nearest);
    let mut original_params = DrawTextureParams::default();
    original_params.dest_size = Some(Vec2 {
        x: original.width() * scale_factor as f32,
        y: original.height() * scale_factor as f32,
    });

    let scaled = load_xbrz_texture("res/hut.png".to_string(), scale_factor).await;
    let scaled_x_pos: f32 = (original.width() * scale_factor as f32) + 12.0;

    loop {
        clear_background(BLACK);

        draw_texture_ex(&original, 0., 0., WHITE, original_params.clone());
        draw_texture(&scaled, scaled_x_pos, 0., WHITE);

        next_frame().await
    }
}
