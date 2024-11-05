pub fn stretch_image(view_width: u32, view_height: u32, image_width: u32, image_height: u32) -> (f32, f32, f32, f32) {
    let width_rate = image_width as f32 / view_width as f32;
    let height_rate = image_height as f32 / view_height as f32;
    let max_rate = width_rate.max(height_rate);
    let min_rate = width_rate.min(height_rate);

    let max = max_rate / min_rate;
    let offset = (max - 1f32) / 2f32 / max;

    match width_rate >= height_rate {
        true => {
            (offset, 1f32 - offset, 0f32, 1f32)
        },
        false => {
            (0f32, 1f32, offset, 1f32 - offset)
        }
    }
}