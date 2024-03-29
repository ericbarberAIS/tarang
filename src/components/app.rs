use yew::html::Scope;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

            <div class="container">
                <main>
                    <Switch<Route> render={switch} />
                </main>
            </div>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and images from " }
                        <a href="https://unsplash.com">{ "Unsplash" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Tarang" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                            { "About Us" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::EventListing}>
                            { "Events" }
                        </Link<Route>>

                        <Link<Route> classes={classes!("navbar-item")} to={Route::Contact}>
                            { "Contact" }
                        </Link<Route>>


                        // Navbar drop down
                        // <div class="navbar-item has-dropdown is-hoverable">
                        //     <div class="navbar-link">
                        //         { "More" }
                        //     </div>
                        //     <div class="navbar-dropdown">
                        //         <Link<Route> classes={classes!("navbar-item")} to={Route::Authors}>
                        //             { "Meet the authors" }
                        //         </Link<Route>>
                        //     </div>
                        // </div>
                        // end of drop down menu
                    </div>
                </div>
            </nav>
        }
    }
}
