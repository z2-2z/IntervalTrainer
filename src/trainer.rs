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
    let audio_files: &[Asset] = match CONFIG().instrument {
        Instrument::Piano => instrument::PIANO.as_ref(),
    };
    let interval_list = match CONFIG().difficulty {
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
    let round = use_signal(|| 0usize); // just exists to signal re-renders to this component
    
    rsx! {
        div {
            class: "tabs is-centered pt-2 is-size-6",
            
            ul {
                li {
                    class: "is-active",
                    Link {
                        to: "",
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
                    Link {
                        to: Route::SettingsView {},
                        "Settings"
                    }
                }
            }
        }
        
        IntervalGuesser {
            round,
            ascending,
            interval,
            first,
            second,
        }
        
        // Always keep to signal refresh:
        p {
            display: "none",
            "{round()}"
        }
    }
}

#[inline(always)]
fn button_class(disabled: bool) -> &'static str {
    if disabled {
        "is-danger is-outlined"
    } else {
        ""
    }
}

struct GuesserState {
    streak: usize,
    wrong: bool,
    disabled: Vec<bool>,
}

impl GuesserState {
    fn new(num_intervals: usize) -> Self {
        Self {
            streak: 0,
            wrong: false,
            disabled: vec![false; num_intervals],
        }
    }
    
    fn wrong(&mut self, idx: usize) {
        self.wrong = true;
        self.streak = 0;
        self.disabled[idx] = true;
    }
    
    fn right(&mut self) {
        self.wrong = false;
        self.streak += 1;
        for v in &mut self.disabled {
            *v = false;
        }
    }
}

#[component]
fn IntervalGuesser(round: Signal<usize>, ascending: bool, interval: usize, first: usize, second: usize) -> Element {
    let audio_files: &[Asset] = match CONFIG().instrument {
        Instrument::Piano => instrument::PIANO.as_ref(),
    };
    let interval_list = match CONFIG().difficulty {
        Difficulty::Basic => BASIC_INTERVALS.as_ref(),
        Difficulty::Advanced => ADVANCED_INTERVALS.as_ref(),
    };
    
    let mut state = use_signal(|| GuesserState::new(interval_list.len()));
    
    rsx! {
        div {
            class: "block",
            
            h1 {
                class: "subtitle is-1 has-text-centered",
                
                span {
                    "{state.read().streak}"
                }
            }
        }
        
        div {
            class: "block",
        
            div {
                class: "columns is-fullwidth mx-2",
                
                div {
                    class: "column is-full",
                    style: "height: 250px;",
                    
                    button {
                        class: "button is-fullwidth is-large",
                        style: "height: 100%;",
                        
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
                                    }, 1000);
                                    setTimeout(() => {
                                        first.pause();
                                        second.pause();
                                        first.currentTime = 0;
                                        second.currentTime = 0;
                                    }, 2500);
                                "#
                            ).await.expect("Eval JS code failed");
                        },
                        
                        span {
                            img {
                                src: asset!("/assets/icons/notes.png"),
                            }
                        }
                    }
                }
                
                div {
                    class: "column is-full fixed-grid has-2-cols",
                    
                    div {
                        class: "grid is-gap-0",
                        
                        for (idx, i) in interval_list.iter().enumerate() {
                            div {
                                class: "cell",
                                
                                button {
                                    key: "interval-{i}",
                                    class: "button is-large is-fullwidth {button_class(state.read().disabled[idx])}",
                                    disabled: state.read().disabled[idx],
                                    
                                    onclick: move |_| {
                                        if *i == interval {
                                            /* Update stats */                                            
                                            let stats = &mut CONFIG.write().stats;
                                            if !state.read().wrong {
                                                stats.right[interval - 1] += 1;
                                            } else {
                                                stats.wrong[interval - 1] += 1;
                                            }
                                            stats.total += 1;
                                            
                                            /* Reset state */
                                            state.write().right();
                                            
                                            /* Signal refresh to get new interval */
                                            *round.write() += 1;
                                        } else {
                                            state.write().wrong(idx);
                                        }
                                    },
                                    
                                    "{interval_name(*i)}"
                                }
                            }
                        }
                    }
                }
            }
        }
        
        div {
            display: "none",
            
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
        }
    }
}
