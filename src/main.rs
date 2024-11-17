use std::time::Duration;

use chrono::{DateTime, Local, Utc};
use leptos::{
    component, leptos_dom::helpers::IntervalHandle, set_interval_with_handle, view, Effect,
    IntoView, RwSignal, SignalUpdate,
};

fn main() {
    leptos::mount_to_body(|| view! { <Application/> })
}

macro_rules! pluralize {
    ($amount:expr, $word:literal) => {{
        let val = $amount;
        let word = $word;
        if val == 1 {
            format!("{val} {word}")
        } else {
            format!("{val} {word}s")
        }
    }};
}

fn get_sleep(last_sleep: DateTime<Utc>) -> String {
    let now = Local::now().to_utc();
    println!("{now:?}, last_sleep: {last_sleep:?}");

    let diff = now.signed_duration_since(last_sleep);
    let days = diff.num_days();
    let hours = diff.num_hours() - days * 24;
    let minutes = diff.num_minutes() - hours * 60 - days * 24 * 60;
    let seconds = diff.num_seconds() - minutes * 60 - hours * 60 * 60 - days * 24 * 60 * 60;

    let mut parts = vec![];

    if days > 0 {
        parts.push(pluralize!(days, "day"));
    }

    if hours > 0 {
        parts.push(pluralize!(hours, "hour"));
    }

    if minutes > 0 {
        parts.push(pluralize!(minutes, "minute"));
    }

    let seconds = pluralize!(seconds, "second");
    match parts.len() {
        0 => seconds,
        1 => format!("{} and {seconds}", parts[0]),
        _ => format!("{}, and {seconds}", parts.join(", ")),
    }
}

fn get_last_sleep() -> String {
    let last_sleep = DateTime::parse_from_rfc3339("2024-11-17T01:22:00Z")
        .expect("valid time")
        .to_utc();

    get_sleep(last_sleep)
}

#[component]
fn Application() -> impl IntoView {
    let time = RwSignal::new(String::new());

    time.update(|v| *v = get_last_sleep());

    {
        let time = time.clone();
        Effect::new(move |prev_handle: Option<IntervalHandle>| {
            if let Some(ph) = prev_handle {
                ph.clear();
            }

            set_interval_with_handle(
                move || time.update(|v| *v = get_last_sleep()),
                Duration::from_secs(1),
            )
            .expect("valid interval")
        });
    }

    view! {
        <div id="wrapper">
            <div id="main">
                <p class="normal">"It has been about"</p>
                <p id="time">{time}</p>
                <p class="normal">"since Bae last overslept"</p>
            </div>
            <div id="footer">
                <p>
                    "Site source can be found "
                    <a href="https://github.com/Pyreko/jdonmysnore" target="_blank" rel="noopener noreferrer">"on GitHub"</a>
                    "."
                </p>
                <span id="sorry">
                    <p>"This is all meant in good fun, love you Bae! ❤️"</p>
                    <p class="tiny">sorry</p>
                </span>
            </div>
        </div>
    }
}
