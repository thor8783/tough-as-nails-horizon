fn tan_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::tough_as_nails_horizon as m;
    if !game.native_on(m::ID) || !game.native_playing() { return; }
    use horizon_client::extras::Corner;
    let corner = Corner::TopRight;
    if !game.native_corner_shown(corner) { return; }
    let state = m::snapshot();
    let text = format!("Hydration {:>3.0}%  Body {:>3.0}%", state.hydration, state.body_temperature);
    let width = game.native_text_width(&text) + 8.0;
    let (x, y) = corners.place(corner, width, 14.0);
    game.native_fill(batch, [x, y, width, 14.0], 0x7800_0000);
    game.native_text(batch, &text, [x + 4.0, y + 3.0], 0xFFFFFF, true);
}
