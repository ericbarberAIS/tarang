use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Tarang" }</h1>
                        <h2 class="subtitle">{"Tarang welcomes all without regard to racial, ethnic, regional, religious, political, or socio- economic background and its mission seeks:

To make a positive difference in the community by hosting informative cultural programs,
promoting local talents, and supporting local charities
To foster community spirit by creating a place for likeminded cultural enthusiasts to come
together
To promote awareness and understanding of the Indian subcontinent cultural sphere or the Indic
world"}</h2>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img
                            // alt="A random image for the input term 'sound'."
                            // src="https://source.unsplash.com/random/1200x400/?sound"
                            alt="A random image for the input term 'sound'."
                            src="/static/events/imports/jpg/image_4.jpg"
                        />
                    </figure>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                    <p class="title">{ "What is Tarang?" }</p>
                    <p class="subtitle">{ "Discover the harmony of music!" }</p>

                    <div class="content">
                        {r#"
                        Tarang is a volunteer-run initiative for promotion of Indic-centric cultural programs in the Greater Washington DC area. It is a program of the non-profit Shabas Aid Foundation. Countries such as Afghanistan, Bangladesh, India, Iran, Nepal, Pakistan, and Sri Lanka where more than a billion Urdu and Hindi speakers reside, were each historically influenced by Indian culture, which itself formed from the various distinct indigenous cultures of these regions. Tarang aims to bring people from this diaspora together through the power of performing arts including music, dance, multi-media storytelling, and mushaira. At Tarang, we believe that powerful storytelling through arts has the ability to both entertain and inform the diaspora about their shared and often hidden cultural treasures. Stories can unite individuals from all walks of life and create a sense of harmony and connection. The Tarang team is dedicated to curating diverse stories and experiences that celebrate different cultures, genres, and styles. Join us on this cultural journey and explore the beautiful tapestry of sights and sounds that Tarang has to offer.
                        "#}
                    </div>
                    </div>
                    </div>

                    <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Who are we?" }</p>
                        <div class="content">
                            {r#" We'er a team of Greater Metro Washington DC area based professionals who are also passionate culture enthusiasts, volunteering to bring you the joy of performing arts that transcends boundaries.

                                At Tarang, we strive to provide authentic and immersive musical experiences. Our events are carefully curated to showcase the rich diversity of music from around the world. Join us in celebrating the universal language of music and let the melodies of Tarang resonate within you.
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
