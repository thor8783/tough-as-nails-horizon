fn tan_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::tough_as_nails_horizon as m;
    if mod_id != m::ID || button != 0 { return; }
    let enabled = game.native_setting(m::ID, "simulation").as_deref() != Some("false");
    game.native_set_setting(m::ID, "simulation", if enabled { "false" } else { "true" });
}
