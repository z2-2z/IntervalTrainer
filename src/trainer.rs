use rand::Rng;
use dioxus::prelude::*;
use dioxus::logger::tracing::info;
use crate::{Route, CONFIG, instrument::load_instrument, Difficulty};

const BASIC_INTERVALS: [usize; 7] = [
    2,4,5,7,9,11,12,
];
const ADVANCED_INTERVALS: [usize; 14] = [
    1,2,3,4,5,6,7,8,9,10,11,12,13,14
];

#[component]
pub fn TrainerView() -> Element {
    let audio_files = load_instrument(CONFIG().instrument);
    let round = use_signal(|| 0usize);
    
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/trainer.css") }
        
        Link {
            to: Route::SettingsView {},
            
            "settings"
        }
        
        Link {
            to: Route::StatisticsView {},
            
            "stats"
        }
        
        IntervalGuesser {
            round: round,
            num_files: audio_files.len(),
        }
    }
}

#[component]
fn IntervalGuesser(round: Signal<usize>, num_files: usize) -> Element {
    let mut rng = rand::rng();
    rng.reseed().expect("Could not seed RNG");
    
    let ascending = rng.random::<bool>();
    let interval = match CONFIG().difficulty {
        Difficulty::Basic => BASIC_INTERVALS[rng.random_range(..BASIC_INTERVALS.len())],
        Difficulty::Advanced => ADVANCED_INTERVALS[rng.random_range(..ADVANCED_INTERVALS.len())],
    };
    let start_point = if ascending {
        rng.random_range(..num_files-interval)
    } else {
        rng.random_range(interval..num_files)
    };
    let mut wrong = use_signal(|| false);
    
    info!("Random interval: ascending={} start_point={} interval={}", ascending, start_point, interval);
    
    rsx! {
        h1 {
            "Trainer"
        }
        
        
        
        p {
            "ascending={ascending} start_point={start_point} interval={interval}"
        }
        
        button {
            onclick: move |_| *wrong.write() = true,
            
            "wrong"
        }
        
        button {
            onclick: move |_| {
                let stats = &mut CONFIG.write().stats;
                if !wrong() {
                    stats.streak += 1;
                    stats.right[interval] += 1;
                } else {
                    stats.streak = 0;
                    stats.wrong[interval] += 1;
                }
                *round.write() += 1;
                *wrong.write() = false;
            },
            
            "right"
        }
    }
}
