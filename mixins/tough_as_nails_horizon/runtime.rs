use std::cell::RefCell;

pub const ID: &str = "tough_as_nails_horizon";
pub const NAME: &str = "Tough As Nails: Horizon";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Hydration and environmental exposure survival systems.";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub hydration: f32,
    pub body_temperature: f32,
    pub ticks: u64,
    pub last_drink_request: u64,
}

impl Default for State {
    fn default() -> Self {
        Self { hydration: 100.0, body_temperature: 50.0, ticks: 0, last_drink_request: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Exposure {
    pub altitude: f32,
    pub day_time: i64,
    pub rain: f32,
    pub movement: f32,
    pub sprinting: bool,
    pub swimming: bool,
    pub dimension: &'static str,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn snapshot() -> State { STATE.with(|state| *state.borrow()) }

pub fn restore(hydration: f32, body_temperature: f32) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.hydration = hydration.clamp(0.0, 100.0);
        state.body_temperature = body_temperature.clamp(0.0, 100.0);
    });
}

pub fn step(exposure: Exposure) -> State {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        state.ticks = state.ticks.saturating_add(1);

        let movement = exposure.movement.clamp(0.0, 1.0);
        let exertion = if movement > 0.01 {
            1.0 + movement
                + if exposure.sprinting { 0.8 } else { 0.0 }
                + if exposure.swimming { 0.2 } else { 0.0 }
        } else {
            0.0
        };
        state.hydration = (state.hydration - exertion * 0.0025).max(0.0);

        let target = target_temperature(exposure);
        state.body_temperature += (target - state.body_temperature) * 0.004;
        state.body_temperature = state.body_temperature.clamp(0.0, 100.0);
        *state
    })
}

pub fn drink(amount: f32) -> State {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        state.hydration = (state.hydration + amount.max(0.0)).min(100.0);
        *state
    })
}

pub fn drink_request_seen(request: u64) -> bool {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if request == 0 || request <= state.last_drink_request { return false; }
        state.last_drink_request = request;
        state.hydration = (state.hydration + 35.0).min(100.0);
        true
    })
}

fn target_temperature(e: Exposure) -> f32 {
    if e.dimension == "minecraft:the_nether" { return 88.0; }
    if e.dimension == "minecraft:the_end" { return 48.0; }
    let altitude = ((e.altitude - 64.0) * -0.12).clamp(-20.0, 24.0);
    let phase = e.day_time.rem_euclid(24_000) as f32 / 24_000.0;
    let daylight = (1.0 - ((phase - 0.5).abs() * 2.0)).clamp(0.0, 1.0);
    let sun = (daylight - 0.35) * 12.0;
    let rain = e.rain.clamp(0.0, 1.0) * -8.0;
    (50.0 + altitude + sun + rain).clamp(5.0, 92.0)
}

pub fn settings_labels(enabled: bool) -> Vec<String> {
    vec![format!("Survival simulation: {}", if enabled { "ON" } else { "OFF" })]
}
