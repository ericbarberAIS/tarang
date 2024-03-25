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
                        <h1 class="title is-1 has-text-primary">{ "Tarang" }</h1>
                        <div class="content box">
                            <p class="subtitle">{"Tarang welcomes all without regard to racial, ethnic, regional, religious, political, or socio-economic background."}</p>
                            <p class="subtitle">{"Its mission seeks:"}</p>
                            <ul>
                                <li>{"To make a positive difference in the community by hosting informative cultural programs, promoting local talents, and supporting local charities."}</li>
                                <li>{"To foster community spirit by creating a place for like-minded cultural enthusiasts to come together."}</li>
                                <li>{"To promote awareness and understanding of the Indian subcontinent cultural sphere or the Indic world."}</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img
                            alt="Performance from Tarang."
                            src="/static/home/img/hero_image.jpg"
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
                        <p class="title has-text-primary">{"What is Tarang?"}</p>
                        <p class="subtitle">{"Discover the harmony of music!"}</p>

                        <div class="content">
                            <p>
                                {"Tarang is a volunteer-run initiative for promotion of Indic-centric cultural programs in the Greater Washington DC area."}
                            </p>
                            <p>
                                {"It is a program of the non-profit Shabas Aid Foundation."}
                            </p>
                            <p>
                                {"Countries such as Afghanistan, Bangladesh, India, Iran, Nepal, Pakistan, and Sri Lanka where more than a billion Urdu and Hindi speakers reside, were each historically influenced by Indian culture, which itself formed from the various distinct indigenous cultures of these regions. Tarang aims to bring people from this diaspora together through the power of performing arts including music, dance, multi-media storytelling, and mushaira. At Tarang, we believe that powerful storytelling through arts has the ability to both entertain and inform the diaspora about their shared and often hidden cultural treasures. Stories can unite individuals from all walks of life and create a sense of harmony and connection. The Tarang team is dedicated to curating diverse stories and experiences that celebrate different cultures, genres, and styles. Join us on this cultural journey and explore the beautiful tapestry of sights and sounds that Tarang has to offer."}
                            </p>
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title has-text-primary">{"Who are we?"}</p>
                        <div class="content">
                            <p>
                                {"Tarang is 100% volunteer driven. Our team consists of Greater Metro Washington DC area based accomplished professionals who are also passionate culture enthusiasts, volunteering to bring you the joy of performing arts that transcends boundaries."}
                            </p>
                            <p>
                                {"At Tarang, we strive to provide authentic and immersive performing arts experiences. Our events are carefully curated to showcase both the rich diversity of culture from the Indian subcontinent and the local artists from that diaspora."}
                            </p>
                            <p>
                                {"Join us in celebrating the universal language of arts and let the melodies of Tarang resonate within you."}
                            </p>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
