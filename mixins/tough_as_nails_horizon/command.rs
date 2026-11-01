fn tan_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::tough_as_nails_horizon as m;
    if !game.native_on(m::ID) { return; }
    if command == "tan drink" {
        let nonce = ((game.world.game_time.max(0) as u64) << 20) | (game.ticks & 0xFFFFF);
        game.native_set_setting(m::ID, "drink_request", &nonce.to_string());
        m::drink(35.0);
        game.native_chat("Tough As Nails: hydration restored.");
        *ran = true;
    } else if command == "tan status" {
        let state = m::snapshot();
        game.native_chat(&format!("Hydration: {:.0}% | Body temperature: {:.0}%", state.hydration, state.body_temperature));
        *ran = true;
    }
}
