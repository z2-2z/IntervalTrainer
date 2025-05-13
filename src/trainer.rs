use rand::Rng;
use dioxus::prelude::*;
use crate::{Route, CONFIG, instrument, Difficulty, Instrument};

const BASIC_INTERVALS: [usize; 7] = [
    2,4,5,7,9,11,12,
];
const ADVANCED_INTERVALS: [usize; 14] = [
    1,2,3,4,5,6,7,8,9,10,11,12,13,14
];

fn interval_name(interval: usize) -> &'static str {
    match interval {
        1 => "Minor 2nd",
        2 => "Major 2nd",
        3 => "Minor 3rd",
        4 => "Major 3rd",
        5 => "Perfect 4th",
        6 => "Tritone",
        7 => "Perfect 5th",
        8 => "Minor 6th",
        9 => "Major 6th",
        10 => "Minor 7th",
        11 => "Major 7th",
        12 => "Octave",
        13 => "Minor 9th",
        14 => "Major 9th",
        _ => unreachable!(),
    }
}

#[component]
pub fn TrainerView() -> Element {
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
        }
    }
}

#[component]
fn IntervalGuesser(round: Signal<usize>) -> Element {
    let audio_files: &[Asset] = match CONFIG().instrument {
        Instrument::Piano => instrument::PIANO.as_ref(),
    };
    let difficulty = CONFIG().difficulty;
    let interval_list = match &difficulty {
        Difficulty::Basic => BASIC_INTERVALS.as_ref(),
        Difficulty::Advanced => ADVANCED_INTERVALS.as_ref(),
    };
    
    let mut rng = rand::rng();
    rng.reseed().expect("Could not seed RNG");
    let ascending = rng.random::<bool>();
    let interval = interval_list[rng.random_range(..interval_list.len())];
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
                            first.pause();
                        }, 750);
                        setTimeout(() => {
                            second.pause();
                            first.currentTime = 0;
                            second.currentTime = 0;
                        }, 2500);
                    "#
                ).await.expect("Eval JS code failed");
            },
            
            "play"
        }
        
        div {
            id: "interval-selection",
            
            for i in interval_list {
                button {
                    key: "interval-{i}",
                    onclick: move |_| {
                        if *i == interval {
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
                        } else {
                            *wrong.write() = true;
                        }
                    },
                    
                    "{interval_name(*i)}"
                }
            }
        }
    }
}
