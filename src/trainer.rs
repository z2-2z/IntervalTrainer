use rand::Rng;
use dioxus::prelude::*;
use crate::{Route, CONFIG, instrument, Difficulty, Instrument};

const BASIC_INTERVALS: [usize; 7] = [
    2,4,5,7,9,11,12,
];
const ADVANCED_INTERVALS: [usize; 14] = [
    1,2,3,4,5,6,7,8,9,10,11,12,13,14
];

#[component]
pub fn TrainerView() -> Element {
    let audio_files: &[Asset] = match CONFIG().instrument {
        Instrument::Piano => instrument::PIANO.as_ref(),
    };
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
            audio_files,
        }
    }
}

#[component]
fn IntervalGuesser(round: Signal<usize>, audio_files: &'static [Asset]) -> Element {
    let difficulty = CONFIG().difficulty;
    let mut rng = rand::rng();
    rng.reseed().expect("Could not seed RNG");
    let ascending = rng.random::<bool>();
    let interval = match &difficulty {
        Difficulty::Basic => BASIC_INTERVALS[rng.random_range(..BASIC_INTERVALS.len())],
        Difficulty::Advanced => ADVANCED_INTERVALS[rng.random_range(..ADVANCED_INTERVALS.len())],
    };
    let (first, second) = if ascending {
        let first = rng.random_range(..audio_files.len() - interval);
        (first, first + interval)
    } else {
        let first = rng.random_range(interval..audio_files.len());
        (first, first - interval)
    };
    let mut wrong = use_signal(|| false);

    rsx! {
        h1 {
            "Trainer"
        }
        
        p {
            "ascending={ascending} firs={first} second={second} interval={interval}"
        }
        
        audio {
            src: audio_files[first],
            id: "audio-first",
            controls: false,
            autoplay: false,
            display: "none",
        }
        
        audio {
            src: audio_files[second],
            id: "audio-second",
            controls: false,
            autoplay: false,
            display: "none",
        }
        
        button {
            onclick: move |_| async move {
                document::eval(
                    r#"
                        const first = document.getElementById("audio-first");
                        const second = document.getElementById("audio-second");
                        first.play();
                        setTimeout(() => {
                            second.play();
                        }, 750);
                        setTimeout(() => {
                            first.pause();
                            first.currentTime = 0;
                            second.pause();
                            second.currentTime = 0;
                        }, 2000);
                    "#
                ).await.expect("Eval JS code failed");
            },
            
            "play"
        }
        
        match &difficulty {
            Difficulty::Basic => {
                
            },
            Difficulty::Advanced => {},
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
                    stats.right[interval - 1] += 1;
                } else {
                    stats.streak = 0;
                    stats.wrong[interval - 1] += 1;
                }
                *round.write() += 1;
                *wrong.write() = false;
            },
            
            "right"
        }
    }
}
