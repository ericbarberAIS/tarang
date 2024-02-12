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
                        <div class="content box">
                            <p class="subtitle">{"Tarang welcomes all without regard to racial, ethnic, regional, religious, political, or socio-economic background. Its mission seeks:"}</p>
                            <ul>
                                <li>{"To make a positive difference in the community by hosting informative cultural programs, promoting local talents, and supporting local charities."}</li>
                                <li>{"To foster community spirit by creating a place for like-minded cultural enthusiasts to come together."}</li>
                                <li>{"To promote awareness and understanding of the Indian subcontinent cultural sphere or the Indic world."}</li>
                            </ul>
                        </div>
        //<h2 class="subtitle">{"Tarang welcomes all without regard to racial, ethnic, regional, religious, political, or socio- economic background and its mission seeks:
        //
        // To make a positive difference in the community by hosting informative cultural programs,
        // promoting local talents, and supporting local charities
        // To foster community spirit by creating a place for likeminded cultural enthusiasts to come
        // together
        // To promote awareness and understanding of the Indian subcontinent cultural sphere or the Indic
        // world"}</h2
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
                        <p class="title">{"What is Tarang?"}</p>
                        <p class="subtitle">{"Discover the harmony of music!"}</p>

                        <div class="content">
                            <p>
                                {"Tarang is a volunteer-run initiative dedicated to the promotion of Indic-centric cultural programs in the Greater Washington DC area. It operates under the auspices of the non-profit Shabas Aid Foundation. Our focus is on countries such as Afghanistan, Bangladesh, India, Iran, Nepal, Pakistan, and Sri Lanka. These nations, home to more than a billion Urdu and Hindi speakers, share a rich historical influence from Indian culture, which in turn is a mosaic of the diverse indigenous cultures of these regions."}
                            </p>
                            <p>
                                {"Tarang's mission is to unite this diaspora through the power of performing arts, including music, dance, multimedia storytelling, and mushaira. We believe in the transformative power of storytelling through arts—to entertain, inform, and reveal the shared, often hidden cultural treasures of our community. Stories have the power to unite people from all walks of life, fostering a sense of harmony and connection."}
                            </p>
                            <p>
                                {"The Tarang team is committed to curating a diverse array of stories and experiences that celebrate the myriad cultures, genres, and styles within the Indic diaspora. We invite you to join us on this cultural journey. Explore the beautiful tapestry of sights and sounds that Tarang has to offer, and let's celebrate our shared heritage together."}
                            </p>
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{"Who are we?"}</p>

                        <div class="content">
                            <p>
                                {"We are a team of professionals based in the Greater Metro Washington DC area, united by our passion for culture and the performing arts. As volunteers, we dedicate our efforts to bringing you joy through performances that transcend boundaries."}
                            </p>
                            <p>
                                {"At Tarang, we are committed to providing authentic and immersive musical experiences. Our events are thoughtfully curated to showcase the rich diversity of music from across the globe. Join us in celebrating the universal language of music. Let the melodies of Tarang resonate within you, bridging hearts and cultures."}
                            </p>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
