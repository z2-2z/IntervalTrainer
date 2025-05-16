use dioxus::prelude::*;

mod instrument;
mod settings;
mod trainer;
mod statistics;
use settings::SettingsView;
use trainer::TrainerView;
use statistics::StatisticsView;

pub const BASIC_INTERVALS: [usize; 7] = [
    2,4,5,7,9,11,12,
];
pub const ADVANCED_INTERVALS: [usize; 14] = [
    1,2,3,4,5,6,7,8,9,10,11,12,13,14
];

pub fn interval_name(interval: usize) -> &'static str {
    match interval {
        1 => "Minor 2nd",
        2 => "Major 2nd",
        3 => "Minor 3rd",
        4 => "Major 3rd",
        5 => "Perfect 4th",
        6 => "Tritone",
        7 => "Perfect 5th",
        8 => "Minor 6th",
        9 => "Major 6th",
        10 => "Minor 7th",
        11 => "Major 7th",
        12 => "Octave",
        13 => "Minor 9th",
        14 => "Major 9th",
        _ => unreachable!(),
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
struct Stats {
    right: [usize; 14],
    wrong: [usize; 14],
    total: usize,
    streak: usize,
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

#[allow(clippy::enum_variant_names)]
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    TrainerView,
    
    #[route("/settings")]
    SettingsView,
    
    #[route("/statistics")]
    StatisticsView,
}

fn main() {
    rand::rng().reseed().expect("Could not seed RNG");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/bulma.css") }
        document::Stylesheet { href: asset!("/assets/styles/main.css") }
        
        div {
            "data-theme": "light",
            class: "theme-light container is-max-tablet",
            "style": "height: 100%;",
            
            Router::<Route> {}
        }
    }
}
