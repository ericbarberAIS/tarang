use crate::components::anchor_link::AnchorLink;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use web_sys::{console, window};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventListingElement {
    title: String,
    hero_image_url: String,
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
    id: u32,
}

#[derive(Properties, PartialEq)]
struct EventListingProps {
    event: EventListingElement,
}

#[function_component(EventComponent)]
fn event_component(props: &EventListingProps) -> Html {
    let event = &props.event;

    // Create an `onclick` callback that navigates to the event page
    let handle_click = {
        let event_id = event.id;
        Callback::from(move |_| {
            if let Some(window) = window() {
                let url = format!("/events/{}", event_id);
                window
                    .location()
                    .set_href(&url)
                    .expect("failed to redirect");
            }
        })
    };

    html! {
        <div class="box" onclick={handle_click}>
            // Display the hero image if the URL is not empty
            if !event.hero_image_url.is_empty() {
                <figure class="image">
                    <img src={event.hero_image_url.clone()} alt={format!("Image for {}", &event.title)} />
                </figure>
            }
            <article class="media">
                <div class="media-content">
                    <div class="content">
                        <h2>
                            <strong>{ &event.title }</strong>
                        </h2>
                    </div>
                </div>
            </article>
        </div>
    }
}

#[function_component(EventListing)]
pub fn event_listing() -> Html {
    // Temporarily using static data
    // let static_events = vec![EventListingElement {
    //     title: "NVBA Celebrates Kobi Pronam".to_string(),
    //     hero_image_url: "/static/events/imports/jpg/image_1.jpg".to_string(),
    //     // Assuming the time is in milliseconds since the Unix epoch
    //     time: Utc.timestamp_millis(1686423600000),
    // }];
    //
    // // Convert static data to a state to mimic the original structure
    // let events = use_state(|| static_events);

    let events = use_state(|| {
        // Load and parse the JSON data at compile time
        let data_str = include_str!("../../static/events/list.json");
        console::log_1(&format!("Loaded data: {}", data_str).into());
        serde_json::from_str::<Vec<EventListingElement>>(data_str).unwrap_or_else(|_| Vec::new())
    });

    // Serialize and log the events data for debugging
    if let Ok(events_json) = to_string(&*events) {
        console::log_1(&format!("Event data: {}", events_json).into());
    } else {
        console::log_1(&"Failed to serialize events".into());
    }

    // Separate into upcoming and past events
    let (upcoming_events, past_events): (Vec<&EventListingElement>, Vec<&EventListingElement>) =
        events.iter().partition(|e| e.time > Utc::now());

    html! {
        <>

        <nav class="panel">
            <p class="panel-heading">
                {"Quick Links"}
            </p>
            <div class="panel-block">
                <div class="buttons has-addons">
                    <AnchorLink class="button is-small is-link is-rounded mr-1" target="upcoming-events" label="Upcoming Events" />
                    <AnchorLink class="button is-small is-link is-rounded ml-1" target="past-events" label="Past Events" />
                </div>
            </div>
        </nav>
            <section class="section" id="upcoming-events">
                <h1 class="title">{"Upcoming Events"}</h1>
                <div class="columns is-multiline">
                { for upcoming_events.iter().map(|&event| html! {
                    <div class="column is-half">
                        <EventComponent event={event.clone()} />
                    </div>
                }) }
                </div>
            </section>
            <section class="section" id="past-events">
                <h1 class="title">{"Past Events"}</h1>
                <div class="columns is-multiline">
                { for past_events.iter().map(|&event| html! {
                    <div class="column is-half">
                        <EventComponent event={event.clone()} />
                    </div>
                }) }
                </div>
            </section>
        </>
    }
}
