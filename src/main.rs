use dioxus::prelude::*;

mod instrument;
mod settings;
mod trainer;
mod statistics;
use settings::SettingsView;
use trainer::TrainerView;
use statistics::StatisticsView;

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
