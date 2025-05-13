use dioxus::prelude::*;
use crate::{CONFIG, Difficulty, Route, Instrument};

#[component]
pub fn SettingsView() -> Element {
    let mut diff = use_signal(|| CONFIG().difficulty);
    let mut inst = use_signal(|| CONFIG().instrument);
    let mut set_difficulty = move |difficulty| {
        CONFIG.write().difficulty = difficulty;
        diff.set(difficulty);
    };
    let mut set_instrument = move |instrument| {
        CONFIG.write().instrument = instrument;
        inst.set(instrument);
    };
    
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/settings.css") }
        
        Link {
            to: Route::TrainerView {},
            
            "back"
        }
        div {
            id: "difficulty-selector",
            
            h1 {
                "Select Difficulty"
            }
            button {
                class: if diff() != Difficulty::Basic {
                    "selectable-difficulty"
                } else {
                    "selected-difficulty"
                },
                onclick: move |_| set_difficulty(Difficulty::Basic),
                
                "basic"
            }
            button {
                class: if diff() != Difficulty::Advanced {
                    "selectable-difficulty"
                } else {
                    "selected-difficulty"
                },
                onclick: move |_| set_difficulty(Difficulty::Advanced),
                
                "advanced"
            }
        }
        div {
            id: "instrument-selector",
            
            h1 {
                "Select Instrument"
            }
            button {
                class: if inst() == Instrument::Piano {
                    "selected-instrument"
                } else {
                    "selectable-instrument"
                },
                onclick: move |_| set_instrument(Instrument::Piano),
                
                "Piano"
            }
        }
    }
}
