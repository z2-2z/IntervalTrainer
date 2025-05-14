use dioxus::prelude::*;
use crate::{CONFIG, Route};

#[component]
pub fn StatisticsView() -> Element {
    let stats = CONFIG().stats.clone();
    
    rsx! {
        Link {
            to: Route::TrainerView {},
            
            "back"
        }
        
        p {
            "{stats:#?}"
        }
    }
}
