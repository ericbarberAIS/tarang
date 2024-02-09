use yew::prelude::*;

pub struct Contact;

impl Component for Contact {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="contact-page hero">

                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Contact Us" }</h1>
                    </div>
                </div>

                <div class="container box">
                    <div>
                        <h3 class="contact-heading">{"Phone"}</h3>
                        <p>{"+123 456 7890"}</p> // Replace with your actual phone number
                    </div>
                    <div>
                        <h3 class="contact-heading">{"Email"}</h3>
                        <p>{"info@tarang.org"}</p> // Replace with your actual email address
                    </div>
                    <div>
                        <h3 class="contact-heading">{"Address"}</h3>
                        <p>{"1234 Street Name, City, Country"}</p> // Replace with your actual address
                    </div>
                    <div>
                        <h3 class="contact-heading">{"Social Media"}</h3>
                        <p>{"Follow us on [Social Media Platforms]"}</p> // Add links to your social media
                    </div>
                </div>
            </div>
        }
    }
}
