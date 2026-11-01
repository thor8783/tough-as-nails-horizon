fn tan_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::tough_as_nails_horizon as m;
    if mod_id != m::ID { return; }
    let enabled = game.native_setting(m::ID, "simulation").as_deref() != Some("false");
    buttons.extend(m::settings_labels(enabled));
}
