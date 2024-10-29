use std::time::Duration;

use chrono::{DateTime, Local, Utc};
use leptos::{
    component, leptos_dom::helpers::IntervalHandle, set_interval_with_handle, view, Effect,
    IntoView, RwSignal, SignalUpdate,
};

const LAST_OVERSLEEP_UNIX: i64 = 1730164881;
const LAST_SLEEP: Option<DateTime<Utc>> = DateTime::from_timestamp(LAST_OVERSLEEP_UNIX, 0);

fn main() {
    leptos::mount_to_body(|| view! { <Application/> })
}

fn get_sleep() -> String {
    let last_sleep = LAST_SLEEP.expect("work");
    let now = Local::now().to_utc();

    let diff = now.signed_duration_since(last_sleep);
    let days = diff.num_days();
    let hours = diff.num_hours() - days * 24;
    let minutes = diff.num_minutes() - hours * 60;
    let seconds = diff.num_seconds() - minutes * 60;

    format!("{days} days, {hours} hours, {minutes} minutes, {seconds} seconds")
}

#[component]
fn Application() -> impl IntoView {
    let time = RwSignal::new(String::new());
    time.update(|v| *v = get_sleep());

    {
        let time = time.clone();
        Effect::new(move |prev_handle: Option<IntervalHandle>| {
            if let Some(ph) = prev_handle {
                ph.clear();
            }

            set_interval_with_handle(
                move || time.update(|v| *v = get_sleep()),
                Duration::from_secs(1),
            )
            .expect("valid interval")
        });
    }

    view! {
        <div id="wrapper">
            <div id="main">
                <p class="normal"> "It has been about" </p>
                <div id="time">
                   <p>
                   {time}
                   </p>
                </div>
                <p class="normal">
                    "since Bae last overslept"
                </p>
            </div>
            <div id="footer">
                <p>
                    "Site made with
                    "<a href="https://www.rust-lang.org/" target="_blank" rel="noopener noreferrer">"Rust"</a>
                    " via "
                    <a href="https://leptos.dev/" target="_blank" rel="noopener noreferrer">"Leptos"</a>" with love in like 20 minutes "
                    <a href="https://github.com/Pyreko/jdonmysnore" target="_blank" rel="noopener noreferrer">"(GitHub)"</a>"."
                </p>
                <p>"Please note that this is all meant in good fun! ❤️"</p>
            </div>
        </div>
    }
}
