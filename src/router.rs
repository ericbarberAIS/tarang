use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::AboutUs;
use crate::pages::contact::Contact;
use crate::pages::event::Event;
use crate::pages::events::EventListing;
use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/about")]
    About,

    #[at("/Contact")]
    Contact,

    #[at("/events/:id")]
    Event { id: u64 },

    #[at("/eventlisting")]
    EventListing,

    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::About => {
            html! { <AboutUs /> }
        }

        Route::Contact => {
            html! { <Contact /> }
        }

        Route::Event { id } => {
            html! { <Event seed={id} /> }
        }

        Route::EventListing => {
            html! { <EventListing /> }
        }

        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
