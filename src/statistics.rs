use dioxus::prelude::*;
use crate::{CONFIG, Route};

#[component]
pub fn StatisticsView() -> Element {
    let stats = CONFIG().stats.clone();
    
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/normalize.css") }
        
        Link {
            to: Route::TrainerView {},
            
            "back"
        }
        
        p {
            "{stats:#?}"
        }
    }
}
