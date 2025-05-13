use dioxus::prelude::*;
use crate::{Route, CONFIG};

#[component]
pub fn TrainerView() -> Element {
    let diff = CONFIG().difficulty;
    let inst = CONFIG().instrument;
    
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/trainer.css") }
        
        Link {
            to: Route::SettingsView {},
            
            "settings"
        }
        
        "Current difficulty is {diff:?}. Current instrument is {inst:?}"
    }
}
