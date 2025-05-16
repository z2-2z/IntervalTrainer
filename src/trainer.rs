use rand::Rng;
use dioxus::prelude::*;
use crate::{Route, CONFIG, instrument, Difficulty, Instrument, BASIC_INTERVALS, ADVANCED_INTERVALS, interval_name};

#[derive(Default)]
struct TrainerState {
    interval: usize,
    first: usize,
    second: usize,
}

impl TrainerState {
    fn shuffle(&mut self, interval_list: &[usize], audio_files: &[Asset]) {
        let mut rng = rand::rng();
        
        let ascending = rng.random::<bool>();
        self.interval = interval_list[rng.random_range(..interval_list.len())];
        if ascending {
            self.first = rng.random_range(..audio_files.len() - self.interval);
            self.second = self.first + self.interval
        } else {
            self.first = rng.random_range(self.interval..audio_files.len());
            self.second = self.first - self.interval;
        };
    }
}

#[component]
pub fn TrainerView() -> Element {
    let state = use_signal(|| {
        let audio_files: &[Asset] = match CONFIG.read().instrument {
            Instrument::Piano => instrument::PIANO.as_ref(),
        };
        let interval_list = match CONFIG.read().difficulty {
            Difficulty::Basic => BASIC_INTERVALS.as_ref(),
            Difficulty::Advanced => ADVANCED_INTERVALS.as_ref(),
        };
        let mut ret = TrainerState::default();
        ret.shuffle(interval_list, audio_files);
        ret
    });
    
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
            random_interval: state,
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
    fn new(num_intervals: usize, streak: usize) -> Self {
        Self {
            streak,
            wrong: false,
            disabled: vec![false; num_intervals],
        }
    }
    
    fn wrong(&mut self, idx: usize) -> usize {
        self.wrong = true;
        self.streak = 0;
        self.disabled[idx] = true;
        self.streak
    }
    
    fn right(&mut self) -> usize {
        self.wrong = false;
        self.streak += 1;
        for v in &mut self.disabled {
            *v = false;
        }
        self.streak
    }
}

#[component]
fn IntervalGuesser(random_interval: Signal<TrainerState>) -> Element {
    let audio_files: &[Asset] = match CONFIG().instrument {
        Instrument::Piano => instrument::PIANO.as_ref(),
    };
    let interval_list = match CONFIG().difficulty {
        Difficulty::Basic => BASIC_INTERVALS.as_ref(),
        Difficulty::Advanced => ADVANCED_INTERVALS.as_ref(),
    };
    
    let mut state = use_signal(|| GuesserState::new(interval_list.len(), CONFIG().stats.streak));
    
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
                                        if *i == random_interval.read().interval {
                                            /* Update stats */                                            
                                            let stats = &mut CONFIG.write().stats;
                                            if !state.read().wrong {
                                                stats.right[random_interval.read().interval - 1] += 1;
                                            } else {
                                                stats.wrong[random_interval.read().interval - 1] += 1;
                                            }
                                            stats.total += 1;
                                            stats.streak = state.write().right();
                                            
                                            /* Signal refresh to get new interval */
                                            random_interval.write().shuffle(interval_list, audio_files);
                                        } else {
                                            CONFIG.write().stats.streak = state.write().wrong(idx);
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
                src: audio_files[random_interval.read().first],
                id: "audio-first",
                controls: false,
                autoplay: false,
                display: "none",
            }
            
            audio {
                src: audio_files[random_interval.read().second],
                id: "audio-second",
                controls: false,
                autoplay: false,
                display: "none",
            }
        }
    }
}
