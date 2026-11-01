fn tan_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::tough_as_nails_horizon as m;
    use horizon_client::extras::Corner;

    if !game.native_on(m::ID) || !game.native_playing() { return; }
    if !game.native_corner_shown(Corner::TopRight) { return; }

    const SCALE: f32 = 2.0;
    const STEP: f32 = 21.0;
    const OUTER: [(u8, u8); 10] = [
        (3, 2), (2, 4), (1, 6), (1, 6), (0, 8),
        (0, 8), (1, 6), (1, 6), (2, 4), (3, 2),
    ];
    const INNER: [(u8, u8); 10] = [
        (0, 0), (3, 2), (2, 4), (2, 4), (1, 6),
        (1, 6), (1, 6), (2, 4), (3, 2), (0, 0),
    ];

    let width = STEP * 10.0 - 1.0;
    let height = SCALE * OUTER.len() as f32;
    let (x, y) = corners.place(Corner::TopRight, width, height);
    let hydration = m::snapshot().hydration.clamp(0.0, 100.0);
    let blue = if hydration <= 15.0 { 0xFFFF_6262 } else if hydration <= 35.0 { 0xFFFF_C95E } else { 0xFF16_8DFF };

    for icon in 0..10 {
        let icon_x = x + icon as f32 * STEP;
        let fraction = (hydration / 10.0 - icon as f32).clamp(0.0, 1.0);
        let filled_rows = (fraction * 8.0).round() as usize;
        for row in 0..OUTER.len() {
            let (outer_start, outer_width) = OUTER[row];
            game.native_fill(
                batch,
                [icon_x + f32::from(outer_start) * SCALE, y + row as f32 * SCALE,
                 f32::from(outer_width) * SCALE, SCALE],
                0xFF0B_1930,
            );

            let (inner_start, inner_width) = INNER[row];
            if inner_width == 0 { continue; }
            let is_filled = row >= 9usize.saturating_sub(filled_rows);
            let color = if is_filled { blue } else { 0xFF3C_526C };
            let inner_x = icon_x + f32::from(inner_start) * SCALE;
            let inner_y = y + row as f32 * SCALE;
            game.native_fill(batch, [inner_x, inner_y, f32::from(inner_width) * SCALE, SCALE], color);
            if is_filled && inner_width >= 4 {
                game.native_fill(batch, [inner_x + SCALE, inner_y, SCALE, SCALE], 0xFF83_CEFF);
            }
        }
    }
}
