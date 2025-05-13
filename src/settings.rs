use dioxus::prelude::*;
use crate::{CONFIG, Difficulty, Route};

#[component]
pub fn SettingsView() -> Element {
    let set_difficulty = |difficulty| CONFIG.write().difficulty = difficulty;
    
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
                onclick: move |_| set_difficulty(Difficulty::Basic),
                
                "basic"
            }
            button {
                id: "difficulty-advanced",
                onclick: move |_| set_difficulty(Difficulty::Advanced),
                
                "advanced"
            }
        }
    }
}
