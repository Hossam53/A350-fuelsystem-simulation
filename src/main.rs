use druid::{AppLauncher, Widget, WindowDesc, widget::{Button, Flex, Label, Slider, ProgressBar}};
use druid::{Data, Lens, WidgetExt};
use rand::Rng;

#[derive(Clone, Data, Lens)]
struct AppState {
    n1_level: f64,
    n2_level: f64,
    altitude: f64,
    center_tank_volume: f64,
    left_wing_tank_volume: f64,
    right_wing_tank_volume: f64,
    time_elapsed: f64,
    fuel_burn_rate: f64,
    payload: f64,  // in tonnes
}

fn main() {
    let main_window = WindowDesc::new(ui_builder()).title("Fuel System Simulation for A350-1000");
    AppLauncher::with_window(main_window)
        .launch(AppState {
            n1_level: 0.5,
            n2_level: 0.5,
            altitude: 10000.0,
            center_tank_volume: 100000.0,  // Center tank can hold up to 100,000 liters
            left_wing_tank_volume: 56000.0,  // Wing tanks can hold up to 56,000 liters each
            right_wing_tank_volume: 56000.0,
            time_elapsed: 0.0,
            fuel_burn_rate: 0.0,
            payload: 0.0,
        })
        .expect("Failed to launch application");
}

fn ui_builder() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("N1 Level:"))
                .with_child(Slider::new().with_range(0.0, 1.0).lens(AppState::n1_level))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2}%", data.n1_level * 100.0)))
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("N2 Level:"))
                .with_child(Slider::new().with_range(0.0, 1.0).lens(AppState::n2_level))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2}%", data.n2_level * 100.0)))
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Altitude (ft):"))
                .with_child(Slider::new().with_range(0.0, 40000.0).lens(AppState::altitude))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.0} ft", data.altitude)))
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Payload (tonnes):"))
                .with_child(Slider::new().with_range(0.0, 319.0).lens(AppState::payload))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2} tonnes", data.payload)))
        )
        .with_child(
            Button::new("Simulate").on_click(|_ctx, data: &mut AppState, _env| {
                let burn_rate = calculate_fuel_burn(data.n1_level, data.n2_level, data.altitude, data.payload);
                data.fuel_burn_rate = burn_rate;
                simulate_fuel_burn(data, burn_rate);
                data.time_elapsed += 1.0;  // Assuming each simulation step is 1 second
            })
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Center Tank Volume:"))
                .with_child(ProgressBar::new().lens(AppState::center_tank_volume))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2} liters", data.center_tank_volume)))
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Left Wing Tank Volume:"))
                .with_child(ProgressBar::new().lens(AppState::left_wing_tank_volume))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2} liters", data.left_wing_tank_volume)))
        )
        .with_child(
            Flex::row()
                .with_child(Label::new("Right Wing Tank Volume:"))
                .with_child(ProgressBar::new().lens(AppState::right_wing_tank_volume))
                .with_spacer(8.0)
                .with_child(Label::dynamic(|data: &AppState, _env| format!("{:.2} liters", data.right_wing_tank_volume)))
        )
        .with_child(
            Label::dynamic(|data: &AppState, _env| format!("Time Elapsed: {:.2} seconds", data.time_elapsed))
        )
        .with_child(
            Label::dynamic(|data: &AppState, _env| format!("Fuel Burn Rate: {:.2} liters/second", data.fuel_burn_rate))
        )
}

fn calculate_fuel_burn(n1_level: f64, n2_level: f64, altitude: f64, payload: f64) -> f64 {
    let mtow = 319.0;  // Maximum Takeoff Weight in tonnes
    let fuel_weight = (mtow - payload) * 0.8;  // Assuming fuel density is 0.8 tonnes per 1000 liters
    let max_payload =   319_000.0 - (100_000.0 * 0.8 + 2.0 * 56_000.0 * 0.8);  // Assuming fuel has a density of 0.8 kg/l


    // Adjust the calculation based on payload and fuel weight
    let weight_factor = 1.0 - (payload + fuel_weight) / mtow;

    n1_level * (1.0 - altitude / 40000.0) * 0.5 + n2_level * (1.0 - altitude / 40000.0) * 0.5 + (payload / max_payload) * 0.5

}

fn simulate_fuel_burn(data: &mut AppState, burn_rate: f64) {
    // Distribute the fuel burn across the three tanks
    let center_burn = burn_rate * 0.5;
    let left_burn = burn_rate * 0.25;
    let right_burn = burn_rate * 0.25;

    data.center_tank_volume -= center_burn;
    data.left_wing_tank_volume -= left_burn;
    data.right_wing_tank_volume -= right_burn;

    // Ensure volumes don't go negative
    if data.center_tank_volume < 0.0 { data.center_tank_volume = 0.0; }
    if data.left_wing_tank_volume < 0.0 { data.left_wing_tank_volume = 0.0; }
    if data.right_wing_tank_volume < 0.0 { data.right_wing_tank_volume = 0.0; }
}

// ... Rest of the code (FuelSystem, Tank, Pump, Valve) ...



// ... Rest of the code (FuelSystem, Tank, Pump, Valve) ...

pub struct FuelSystem {
    center_tank: Tank,
    left_inner_tank: Tank,
    left_outer_tank: Tank,
    right_inner_tank: Tank,
    right_outer_tank: Tank,
    crossfeed_valve: Valve,
    pumps: Vec<Pump>,
}

impl FuelSystem {
    pub fn new() -> Self {
        FuelSystem {
            center_tank: Tank::new(24000.0),
            left_inner_tank: Tank::new(15000.0),
            left_outer_tank: Tank::new(5000.0),
            right_inner_tank: Tank::new(15000.0),
            right_outer_tank: Tank::new(5000.0),
            crossfeed_valve: Valve::new(),
            pumps: vec![
                Pump::new(),
                Pump::new(),
                Pump::new(),
                Pump::new(),
                Pump::new(),
            ],
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update tanks, pumps, valves, etc.
        // Transfer fuel, check for failures, etc.
    }
}

struct Tank {
    capacity: f32,
    current_volume: f32,
}

impl Tank {
    fn new(capacity: f32) -> Self {
        Tank {
            capacity,
            current_volume: 0.0,
        }
    }

    fn add_fuel(&mut self, volume: f32) {
        self.current_volume += volume;
        if self.current_volume > self.capacity {
            self.current_volume = self.capacity;
        }
    }

    fn remove_fuel(&mut self, volume: f32) {
        self.current_volume -= volume;
        if self.current_volume < 0.0 {
            self.current_volume = 0.0;
        }
    }
}

struct Pump {
    is_active: bool,
}

impl Pump {
    fn new() -> Self {
        Pump { is_active: false }
    }

    fn activate(&mut self) {
        self.is_active = true;
    }

    fn deactivate(&mut self) {
        self.is_active = false;
    }
}

struct Valve {
    is_open: bool,
}

impl Valve {
    fn new() -> Self {
        Valve { is_open: false }
    }

    fn open(&mut self) {
        self.is_open = true;
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}
