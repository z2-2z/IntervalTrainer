use dioxus::prelude::*;
use crate::{CONFIG, Route, Stats, interval_name};

#[inline(always)]
fn stats_score(stats: &Stats, interval: usize) -> isize {
    stats.right[interval] as isize - stats.wrong[interval] as isize
}

fn sort_by_score(stats: &Stats) -> Vec<usize> {
    let mut ret = Vec::new();
    
    for i in 0..stats.wrong.len() {
        if stats.right[i] != 0 || stats.wrong[i] != 0 {
            ret.push(i);
        }
    }
    
    if !ret.is_empty() {
        let mut modified;
    
        loop {
            modified = false;
            
            for i in 0..ret.len() - 1 {
                let first = stats_score(stats, ret[i]);
                let second = stats_score(stats, ret[i + 1]);
                
                if second < first {
                    ret.swap(i, i + 1);
                    modified = true;
                }
            }
            
            if !modified {
                break;
            }
        }
    }
    
    ret
}

fn stats_pct<const RIGHT: bool>(stats: &Stats, interval: usize) -> f64 {
    let total = stats.wrong[interval] + stats.right[interval];
    
    if RIGHT {
        stats.right[interval] as f64 * 100.0 / total as f64
    } else {
        stats.wrong[interval] as f64 * 100.0 / total as f64
    }
}

#[component]
pub fn StatisticsView() -> Element {
    let stats = CONFIG().stats.clone();
    let total_right = stats.right.iter().sum::<usize>();
    let score = total_right as f64 * 100.0 / stats.total as f64;
    let intervals = sort_by_score(&stats);
    
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
                    class: "is-active",
                    Link {
                        to: "",
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
        
        if stats.total == 0 {
            p {
                class: "has-text-centered",
                "no data yet"
            }
        } else {
            h1 {
                class: "subtitle is-4 has-text-centered",
                
                "Score: {score:.00}%"
            }
        
            div {
                class: "columns mx-2",
                
                for interval in intervals {
                    div {
                        class: "column",
                        
                        if stats.wrong[interval] >= stats.right[interval] {
                            p {
                                class: "",
                                "{interval_name(interval + 1)}: {stats_pct::<false>(&stats, interval):.00}% wrong"
                            }
                            
                            progress {
                                class: "progress is-medium is-danger",
                                "value": "{stats.wrong[interval]}",
                                "max": "{stats.right[interval] + stats.wrong[interval]}",
                                
                                "{stats_pct::<false>(&stats, interval):.00}%"
                            }
                        } else {
                            p {
                                class: "",
                                "{interval_name(interval + 1)}: {stats_pct::<true>(&stats, interval):.00}% right"
                            }
                            
                            progress {
                                class: "progress is-medium is-success",
                                "value": "{stats.right[interval]}",
                                "max": "{stats.right[interval] + stats.wrong[interval]}",
                                
                                "{stats_pct::<true>(&stats, interval):.00}%"
                            }
                        }
                    }
                }
            }    
        }
    }
}
