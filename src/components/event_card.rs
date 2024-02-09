use yew::prelude::*;
use yew_router::components::Link;

use crate::content::EventMeta;
use crate::generator::Generated;
use crate::router::Route;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub seed: u64,
}

pub struct EventCard {
    event: EventMeta,
}
impl Component for EventCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            event: EventMeta::generate_from_seed(ctx.props().seed),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.event = EventMeta::generate_from_seed(ctx.props().seed);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { event } = self;
        html! {
            <div class="card">
                <div class="card-image">
                    <figure class="image is-2by1">
                        <img alt="This event's image" src={event.image_url.clone()} loading="lazy" />
                    </figure>
                </div>
                <div class="card-content">
                    <Link<Route> classes={classes!("title", "is-block")} to={Route::Event { id: event.seed }}>
                        { &event.title }
                    </Link<Route>>
                    // <Link<Route> classes={classes!("subtitle", "is-block")} to={Route::Author { id: post.author.seed }}>
                    //     { &post.author.name }
                    // </Link<Route>>
                </div>
            </div>
        }
    }
}
