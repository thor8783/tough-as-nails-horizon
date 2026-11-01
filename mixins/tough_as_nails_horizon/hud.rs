fn tan_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::tough_as_nails_horizon as m;
    if !game.native_on(m::ID) || !game.native_playing() { return; }
    use horizon_client::extras::Corner;

    let state = m::snapshot();
    if game.native_corner_shown(Corner::BottomRight) {
        draw_hydration(game, batch, corners, state.hydration);
    }
    if game.native_corner_shown(Corner::TopRight) {
        draw_temperature(game, batch, corners, state.body_temperature);
    }
}

fn draw_hydration(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
    hydration: f32,
) {
    use horizon_client::extras::Corner;

    const OUTER: [(f32, f32); 8] = [
        (3.0, 2.0), (2.0, 4.0), (1.0, 6.0), (0.0, 8.0),
        (0.0, 8.0), (1.0, 6.0), (2.0, 4.0), (3.0, 2.0),
    ];
    const INNER: [(f32, f32); 6] = [
        (3.0, 2.0), (2.0, 4.0), (1.0, 6.0),
        (1.0, 6.0), (2.0, 4.0), (3.0, 2.0),
    ];

    let width = 146.0;
    let height = 16.0;
    let (x, y) = corners.place(Corner::BottomRight, width, height);
    game.native_fill(batch, [x, y, width, height], 0x7800_0000);

    let hydration = hydration.clamp(0.0, 100.0);
    let liquid = if hydration <= 15.0 {
        0xFFFF_5B5B
    } else if hydration <= 35.0 {
        0xFFFF_C857
    } else {
        0xFF48_B8FF
    };
    for icon in 0..10 {
        let icon_x = x + 4.0 + icon as f32 * 10.0;
        for (row, (offset, row_width)) in OUTER.iter().copied().enumerate() {
            game.native_fill(
                batch,
                [icon_x + offset, y + 4.0 + row as f32, row_width, 1.0],
                0xFF12_1D28,
            );
        }

        let fraction = (hydration / 10.0 - icon as f32).clamp(0.0, 1.0);
        let filled_rows = (fraction * INNER.len() as f32).round() as usize;
        let first_filled = INNER.len().saturating_sub(filled_rows);
        for (row, (offset, row_width)) in INNER.iter().copied().enumerate() {
            let color = if row >= first_filled { liquid } else { 0xFF54_606C };
            game.native_fill(
                batch,
                [icon_x + offset, y + 5.0 + row as f32, row_width, 1.0],
                color,
            );
        }
    }

    let label = format!("{:>3.0}%", hydration);
    game.native_text(batch, &label, [x + 108.0, y + 4.0], 0xFFFFFF, true);
}

fn draw_temperature(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
    temperature: f32,
) {
    use horizon_client::extras::Corner;

    let width = 88.0;
    let height = 16.0;
    let (x, y) = corners.place(Corner::TopRight, width, height);
    game.native_fill(batch, [x, y, width, height], 0x7800_0000);

    let temperature = temperature.clamp(0.0, 100.0);
    let color = if temperature <= 30.0 {
        0xFF55_BCFF
    } else if temperature >= 70.0 {
        0xFFFF_7650
    } else {
        0xFFFF_D06A
    };
    game.native_fill(batch, [x + 4.0, y + 3.0, 4.0, 8.0], 0xFF12_1D28);
    game.native_fill(batch, [x + 3.0, y + 10.0, 6.0, 4.0], 0xFF12_1D28);
    let filled = (temperature / 100.0 * 7.0).round().clamp(1.0, 7.0);
    game.native_fill(batch, [x + 5.0, y + 10.0 - filled, 2.0, filled], color);
    game.native_fill(batch, [x + 5.0, y + 11.0, 2.0, 2.0], color);

    let label = format!("BODY {:>3.0}%", temperature);
    game.native_text(batch, &label, [x + 13.0, y + 4.0], 0xFFFFFF, true);
}
