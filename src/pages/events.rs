use crate::components::anchor_link::AnchorLink;
use crate::router::Route;

use chrono::serde::ts_milliseconds;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::console;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct EventListingElement {
    id: u64,
    title: String,
    hero_image_url: String,
    #[serde(with = "ts_milliseconds")]
    time: DateTime<Utc>,
    status: String,
}

#[derive(Properties, PartialEq)]
struct EventListingProps {
    event: EventListingElement,
}

#[function_component(EventComponentTile)]
fn event_component(props: &EventListingProps) -> Html {
    let event = &props.event;
    let event_id = event.id.clone();

    html! {
        <div class="box" >
            // Display the hero image if the URL is not empty
            if !event.hero_image_url.is_empty() {
                <figure class="image">
                    <img src={event.hero_image_url.clone()} alt={format!("Image for {}", &event.title)} />
                </figure>
            }
            <article class="media">
                <div class="media-content">
                    <div class="content">
                        <h3>
                            { &event.title }
                        </h3>
                    </div>
                </div>
            </article>
        </div>
    }
}

#[function_component(EventComponent)]
fn event_component(props: &EventListingProps) -> Html {
    let event = &props.event;
    let event_id = event.id.clone();

    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&Route::Event { id: event_id }));

    html! {
        <div class="box" onclick={onclick}>
            // Display the hero image if the URL is not empty
            if !event.hero_image_url.is_empty() {
                <figure class="image">
                    <img src={event.hero_image_url.clone()} alt={format!("Image for {}", &event.title)} />
                </figure>
            }
            <article class="media">
                <div class="media-content">
                    <div class="content">
                        <h3>
                            { &event.title }
                        </h3>
                    </div>
                </div>
            </article>
        </div>
    }
}

fn load_default_event_data() -> Result<EventListingElement, Box<dyn std::error::Error>> {
    let data_str = include_str!("../../static/events/defaults/list.json");
    let v: Value = serde_json::from_str(data_str)?;

    // Now extract the data manually from the Value `v`
    let id = v["id"].as_u64().ok_or("id should be a u64")? as u64;
    let title = v["title"]
        .as_str()
        .ok_or("title should be a string")?
        .to_owned();
    let hero_image_url = v["hero_image_url"]
        .as_str()
        .ok_or("hero_image_url should be a string")?
        .to_owned();
    let time_str = v["time"].as_str().ok_or("time should be a string")?;
    let time_millis = time_str.parse::<i64>()?;
    let time = Utc.timestamp_millis(time_millis);
    let status = v["inactive"]
        .as_str()
        .ok_or("title should be a string")?
        .to_owned();
    Ok(EventListingElement {
        id,
        title,
        hero_image_url,
        time,
        status,
    })
}

fn default_event() -> EventListingElement {
    EventListingElement {
        id: 0, // Assuming an ID of 0 (or another placeholder value) for the default event
        title: "To be Announced".to_string(),
        hero_image_url: "../../static/events/defaults/no_events_scheduled.webp".to_string(),
        time: Utc::now(), // Placeholder string, adjust according to the actual type you're using
        status: "inactive".to_string(),
    }
}
// Function to load the main events data, handling errors gracefully
fn load_event_data() -> Vec<EventListingElement> {
    let data_str = include_str!("../../static/events/list.json");
    match serde_json::from_str::<Vec<EventListingElement>>(data_str) {
        Ok(events) => events,
        Err(e) => {
            console::log_1(&format!("Error loading events data: {}", e).into());
            Vec::new() // Return an empty vector if there's an error
        }
    }
}

#[function_component(EventListing)]
pub fn event_listing() -> Html {
    // Convert static data to a state to mimic the original structure
    // let events = use_state(|| static_events);
    // Define the default_event function to return a default EventListingElement
    let static_future_event = load_default_event_data();

    // Check if loading the default event was successful.
    // We use 'match' to handle both Ok and Err cases.
    let event_to_display = match static_future_event {
        Ok(event) => event.clone(), // Clone the event if Ok
        Err(_error) => {
            // Handle the error case. For example, you could log the error or
            // create a fallback `EventListingElement`.
            // This is where you would call `default_event()` or similar if you have it.
            default_event()
        }
    };

    // Attempt to load event data
    let events_result = use_state(|| load_event_data());

    // Separate into upcoming and past events
    let (upcoming_events, past_events): (Vec<&EventListingElement>, Vec<&EventListingElement>) =
        events_result
            .iter()
            .filter(|e| e.status == "active")
            .partition(|e| e.time > Utc::now());

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
                if upcoming_events.is_empty() {
                    <div class="columns is-multiline">
                        <div class="column is-one-third">
                            <EventComponentTile event={event_to_display} />
                        </div>
                    </div>
                } else {
                    <div class="columns is-multiline">
                        { for upcoming_events.iter().map(|&event| html! {
                            <div class="column is-one-third">
                                <EventComponent event={event.clone()} />
                            </div>
                        }) }
                    </div>
                }
            </section>
            <section class="section" id="past-events">
                <h1 class="title">{"Past Events"}</h1>
                <div class="columns is-multiline">
                { for past_events.iter().map(|&event| html! {
                    <div class="column is-one-third">
                        <EventComponent event={event.clone()} />
                    </div>
                }) }
                </div>
            </section>
        </>
    }
}
