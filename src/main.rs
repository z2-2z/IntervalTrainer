use dioxus::prelude::*;
use dioxus::logger::tracing::Level;

mod instrument;
mod settings;
mod trainer;
use settings::SettingsView;
use trainer::TrainerView;

#[derive(Clone, Default, Debug, PartialEq)]
struct Stats {
    right: [usize; 14],
    wrong: [usize; 14],
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum Instrument {
    #[default]
    Piano,
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum Difficulty {
    #[default]
    Basic,
    
    Advanced,
}

#[derive(Clone, Default)]
struct AppConfig {
    difficulty: Difficulty,
    instrument: Instrument,
    stats: Stats,
}

static CONFIG: GlobalSignal<AppConfig> = Global::new(AppConfig::default);

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    TrainerView,
    
    #[route("/settings")]
    SettingsView,
}

fn main() {
    #[cfg(debug_assertions)]
    let level = Level::INFO;
    #[cfg(not(debug_assertions))]
    let level = Level::INFO;
    
    dioxus::logger::init(level).expect("logger failed to init");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
