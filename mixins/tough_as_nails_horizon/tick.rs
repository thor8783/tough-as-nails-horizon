fn tan_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::tough_as_nails_horizon as m;
    if !game.native_on(m::ID) { return; }
    if game.native_setting(m::ID, "simulation").as_deref() == Some("false") { return; }

    if m::snapshot().ticks == 0 {
        let hydration = game.native_setting(m::ID, "hydration").and_then(|v| v.parse().ok()).unwrap_or(100.0);
        let temperature = game.native_setting(m::ID, "temperature").and_then(|v| v.parse().ok()).unwrap_or(50.0);
        m::restore(hydration, temperature);
    }

    let Some(player_view) = game.native_player() else { return; };
    let movement = game.player.as_ref().map_or(0.0, |p| {
        ((p.velocity.x * p.velocity.x + p.velocity.z * p.velocity.z).sqrt() as f32 / 0.35).clamp(0.0, 1.0)
    });
    let (sprinting, swimming) = game.player.as_ref().map_or((false, false), |p| (p.sprinting, p.swimming));
    let rain = game.world.weather.levels(1.0).0;
    let state = m::step(m::Exposure {
        altitude: player_view.y as f32,
        day_time: game.world.day_time,
        rain,
        movement,
        sprinting,
        swimming,
        dimension: player_view.dimension,
    });

    if game.world.game_time.rem_euclid(20) == 0 {
        if let Some(request) = game.native_setting(m::ID, "drink_request").and_then(|v| v.parse().ok()) {
            m::drink_request_seen(request);
        }
        if !game.native_window() {
            game.native_set_setting(m::ID, "hydration", &format!("{:.3}", state.hydration));
            game.native_set_setting(m::ID, "temperature", &format!("{:.3}", state.body_temperature));
        }
    }

    if game.native_window() || game.world.game_time.rem_euclid(80) != 0 { return; }
    let Some(player) = game.player.as_mut() else { return; };
    if player.spectator || player.invulnerable { return; }
    let source_name = if state.hydration <= 0.0 {
        Some("minecraft:starve")
    } else if state.body_temperature <= 8.0 {
        Some("minecraft:freeze")
    } else if state.body_temperature >= 94.0 {
        Some("minecraft:on_fire")
    } else { None };
    if let Some(source) = source_name.and_then(|name| game.motion.health.damage.id(name)) {
        let amount = if state.hydration <= 0.0 { 1.0 } else { 0.5 };
        let rules = game.world.health_rules;
        let _ = player.hurt(&game.motion.health, rules, source, amount);
    }
}
