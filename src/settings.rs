use dioxus::prelude::*;
use crate::{CONFIG, Difficulty, Route};

#[component]
pub fn SettingsView() -> Element {
    let mut diff = use_signal(|| CONFIG.with_mut(|c| c.difficulty));
    let mut set_difficulty = move |difficulty| {
        CONFIG.write().difficulty = difficulty;
        diff.set(difficulty);
    };
    
    rsx! {
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
                id: "difficulty-basic",
                class: if diff() != Difficulty::Basic { "selectable-difficulty" } else { "selected-difficulty" },
                onclick: move |_| set_difficulty(Difficulty::Basic),
                
                "basic"
            }
            button {
                id: "difficulty-advanced",
                class: if diff() != Difficulty::Advanced { "selectable-difficulty" } else { "selected-difficulty" },
                onclick: move |_| set_difficulty(Difficulty::Advanced),
                
                "advanced"
            }
        }
    }
}
