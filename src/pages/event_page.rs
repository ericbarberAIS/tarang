use yew::prelude::*;

use crate::content;
use crate::generator::Generated;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct EventPage {
    event: content::Event,
}
impl Component for EventPage {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            event: content::Event::generate_from_seed(ctx.props().seed),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.event = content::Event::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { event } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ &event.name }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "Details" }</p>
                                <div class="tags">
                                    { for event.keywords.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span> }) }
                                </div>
                            </article>
                        </div>
                        <div class="tile is-parent">
                            <figure class="tile is-child image is-square">
                                <img alt="The event's theme picture." src={event.image_url.clone()} />
                            </figure>
                        </div>
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "About the event" }</p>
                                    <div class="content">
                                        { "This event has not been planned yet, please check back at a latter time." }
                                    </div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

// use anyhow::Error;
// use chrono::NaiveDateTime;
// use serde::Deserialize;
// // use yew::format::{Json, Nothing};
// use yew::prelude::*;
// // use yew::services::fetch::{FetchService, FetchTask, Request, Response};
//
// #[derive(Clone, Debug, Deserialize, PartialEq)]
// pub struct EventData {
//     pub id: u64,
//     pub title: String,
//     pub date: NaiveDateTime,
//     pub location: String,
//     pub description: String,
//     pub image_url: String,
//     pub keywords: Vec<String>,
//     // Add other fields as needed...
// }
//
// #[derive(Clone, Debug, Eq, PartialEq, Properties)]
// pub struct Props {
//     pub id: u64,
// }
//
// pub enum Msg {
//     FetchReady(Result<Vec<EventData>, Error>),
// }
//
// pub struct EventPage {
//     fetch_task: Option<FetchTask>,
//     event: Option<EventData>,
// }
//
// impl Component for EventPage {
//     type Message = Msg;
//     type Properties = Props;
//
//     fn create(ctx: &Context<Self>) -> Self {
//         let callback =
//             ctx.link()
//                 .callback(|response: Response<Json<Result<Vec<EventData>, Error>>>| {
//                     let Json(data) = response.into_body();
//                     Msg::FetchReady(data)
//                 });
//
//         let request = Request::get("/static/events/events.json")
//             .body(Nothing)
//             .expect("Could not build request.");
//         let task = FetchService::fetch(request, callback).expect("Failed to start request");
//
//         Self {
//             fetch_task: Some(task),
//             event: None,
//         }
//     }
//
//     fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
//         // fn changed(&mut self, ctx: &Context<Self>) -> bool {
//         self.fetch_task = Some(
//             FetchService::fetch(
//                 Request::get("/static/events/events.json")
//                     .body(Nothing)
//                     .expect("Could not build request."),
//                 ctx.link()
//                     .callback(|response: Response<Json<Result<Vec<EventData>, Error>>>| {
//                         let Json(data) = response.into_body();
//                         Msg::FetchReady(data)
//                     }),
//             )
//             .expect("Failed to start request"),
//         );
//         true
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::FetchReady(response) => {
//                 match response {
//                     Ok(data) => {
//                         self.event = data.into_iter().find(|e| e.id == ctx.props().id);
//                     }
//                     Err(_) => {
//                         self.event = None;
//                     }
//                 }
//                 true
//             }
//         }
//     }
//
//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         if let Some(event) = &self.event {
//             html! {
//                 <div class="section container">
//                     <div class="tile is-ancestor is-vertical">
//                         <div class="tile is-parent">
//                             <article class="tile is-child notification is-light">
//                                 <p class="title">{ &event.title }</p>
//                             </article>
//                         </div>
//                         <div class="tile">
//                             <div class="tile is-parent is-3">
//                                 <article class="tile is-child notification">
//                                     <p class="title">{ "Details" }</p>
//                                     <p>{ format!("Date: {}", event.date) }</p>
//                                     <p>{ format!("Location: {}", event.location) }</p>
//                                     <div class="tags">
//                                         { for event.keywords.iter().map(|tag| html! { <span class="tag is-info">{ tag }</span> }) }
//                                     </div>
//                                 </article>
//                             </div>
//                             <div class="tile is-parent">
//                                 <figure class="tile is-child image is-square">
//                                     <img alt="The event's theme picture." src={event.image_url.clone()} />
//                                 </figure>
//                             </div>
//                             <div class="tile is-parent">
//                                 <article class="tile is-child notification is-info">
//                                     <div class="content">
//                                         <p class="title">{ "About the event" }</p>
//                                         <div class="content">
//                                             // Include additional event details here
//                                         </div>
//                                     </div>
//                                 </article>
//                             </div>
//                         </div>
//                     </div>
//                 </div>
//             }
//         } else {
//             html! {
//                 <p>{ "Loading or event not found" }</p>
//             }
//         }
//     }
// }
