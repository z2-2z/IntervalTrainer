use dioxus::prelude::*;
use crate::{Route, CONFIG};

#[component]
pub fn TrainerView() -> Element {
    let diff = CONFIG.with_mut(|c| c.difficulty);
    
    rsx! {
        Link {
            to: Route::SettingsView {},
            
            "settings"
        }
        
        "Current difficulty is {diff:?}"
    }
}
