use wasm_bindgen::JsCast;
use web_sys::Request;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct Event {
    pub id: u64,
    pub title: String,
    pub date: String,
    pub location: String,
    pub description: String,
}

// Function to load events from JSON file
fn load_events(callback: Callback<Result<Vec<Event>, Error>>) -> FetchTask {
    let request = Request::get("/static/events.json")
        .body(Nothing)
        .expect("Could not build request.");

    FetchService::fetch(
        request,
        callback
            .reform(|response: Response<Json<Result<Vec<Event>, Error>>>| response.into_body().0),
    )
    .expect("Failed to start request")
}
