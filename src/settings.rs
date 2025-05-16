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
        div {
            class: "tabs is-centered pt-2 is-size-6",
            
            ul {
                li {
                    Link {
                        to: Route::TrainerView {},
                        "Trainer"
                    }
                }
                li {
                    Link {
                        to: Route::StatisticsView {},
                        "Statistics"
                    }
                }
                li {
                    class: "is-active",
                    Link {
                        to: "",
                        "Settings"
                    }
                }
            }
        }
        
        div {
            class: "block mx-3",
            
            h1 {
                class: "subtitle is-4",
                "Select Difficulty"
            }
            
            div {
                class: "fixed-grid has-2-cols",
                div {
                    class: "grid is-gap-1",
                    
                    button {
                        class: "cell button",
                        disabled: *diff.read() == Difficulty::Basic,
                        onclick: move |_| set_difficulty(Difficulty::Basic),
                        
                        "basic"
                    }
                    
                    button {
                        class: "cell button",
                        disabled: *diff.read() == Difficulty::Advanced,
                        onclick: move |_| set_difficulty(Difficulty::Advanced),
                        
                        "advanced"
                    }
                }
            }
        }
        
        div {
            class: "block mx-3",
            
            h1 {
                class: "subtitle is-4",
                "Select Instrument"
            }
            
            div {
                class: "grid",
                
                button {
                    class: "cell button",
                    disabled: *inst.read() == Instrument::Piano,
                    onclick: move |_| set_instrument(Instrument::Piano),
                    
                    img {
                        src: asset!("/assets/icons/piano.png"),
                    }
                }
            }
        }
    }
}
