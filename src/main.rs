use dioxus::prelude::*;

mod settings;
mod trainer;
use settings::SettingsView;
use trainer::TrainerView;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum Difficulty {
    #[default]
    Basic,
    
    Advanced,
}

#[derive(Clone, Default)]
struct AppConfig {
    difficulty: Difficulty,
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
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
