use yew::prelude::*;

use crate::components::image_carousel::Carousel;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub seed: u64,
}

#[function_component(Event)]
pub fn event(props: &Props) -> Html {
    let images = vec![
        // "/static/events/imports/jpg/event_description.jpg",
        "/static/events/imports/jpg/image_1.jpg",
        "/static/events/imports/jpg/image_2.jpg",
        "/static/events/imports/jpg/image_3.jpg",
        "/static/events/imports/jpg/image_4.jpg",
        "/static/events/imports/jpg/image_5.jpg",
        "/static/events/imports/jpg/image_6.jpg",
        "/static/events/imports/jpg/image_7.jpg",
        "/static/events/imports/jpg/image_8.jpg",
    ];

    let details_image = vec!["/static/events/imports/jpg/event_description.jpg"];

    html! {
        <div class="section container">
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-parent">
                    <article class="tile is-child">
                        <p class="title has-text-primary">{"Tagore and The Sufi Poets: A Journey in Time"}</p>
                    </article>
                </div>
                <div class="tile">
                    <div class="tile is-parent is-4">
                        <article class="tile is-child notification">
                            <p class="title has-text-info">{ "Details" }</p>
                                <div class="content">
                                    <p>{"Date: June l0th, 2023"}</p>
                                    <p>{"Time: 7-10 PM (Free Event)"}</p>
                                    // <!-- Make the address a link - can link to a map or other contact info -->
                                    <p>{"Venue: "}<a href="https://maps.google.com/?q=42149+Greenstone+Dr,+Aldie,+VA+20105" target="_blank">{"Mercer Middle School, 42149 Greenstone Dr, Aldie, VA 20105"}</a></p>
                                </div>

                                // <!-- Presentation Title Section -->
                                <div class="content">
                                    <p>{"A Journey in Time - A Tagore and the Sufi Poets: A multicultural event (narrated in English) celebrating dance, music, and poetry in a rich fusion of Farsi, Hindi and Bengali!"}</p>
                                </div>
                        </article>
                    </div>
                    <div class="tile is-parent">
                        <figure class="tile is-child image">
                            <Carousel images={images} />
                            // <img alt="The event's theme picture." src={event.image_url.clone()} />
                        </figure>
                    </div>
                </div>
                        <div class="tile is-parent">
                        <figure class="tile is-child image">
                            <Carousel images={details_image} />
                            // <img alt="The event's theme picture." src={event.image_url.clone()} />
                        </figure>
                    </div>

                    <div class="tile is-ancestor">
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-white">
                                <div class="content">
                                    // <!-- Use "title" for main heading and "subtitle" for subheadings -->
                                    <p class="title has-text-weight-bold has-text-info">{"Event Announcement"}</p>

                                    // <!-- Event Announcement Section -->
                                    <div class="content">
                                        <p class="subtitle has-text-weight-semibold">{"NVBA Celebrates Kobi Pronam"}</p>
                                        <p>{"Date: June l0th, 2023"}</p>
                                        <p>{"Time: 7-10 PM (Free Event)"}</p>
                                        // <!-- Make the address a link - can link to a map or other contact info -->
                                        <p>{"Venue: "}<a href="https://maps.google.com/?q=42149+Greenstone+Dr,+Aldie,+VA+20105" target="_blank">{"Mercer Middle School, 42149 Greenstone Dr, Aldie, VA 20105"}</a></p>
                                    </div>

                                    // <!-- Presentation Title Section -->
                                    <div class="content">
                                        <p class="subtitle has-text-weight-semibold">{"Tarang DC Presents:"}</p>
                                        <p>{"A Journey in Time - A Tagore and the Sufi Poets: A multicultural event (narrated in English) celebrating dance, music, and poetry in a rich fusion of Farsi, Hindi and Bengali!"}</p>
                                    </div>

                                    // <!-- Did You Know Section -->
                                    <div class="content">
                                        <p class="subtitle has-text-weight-semibold">{"Did You Know?"}</p>
                                        <ul>
                                            <li>{"India's Nobel Laureate poet Rabindranath Tagore shared deep philosophical affinities with noted mystic poets – Hafiz of Shirazi (Iran), Bhakti Sant Kabir of Benares (India), and Baul Lalan Fakir of Kushtia (Bangladesh)"}</li>
                                            <li>{"Farsi was an official language in Bengal for 600 years till 1836?"}</li>
                                            <li>{"Tagore grew up listening to Hafiz’s poems, introduced Kabir to the West, was Lalan’s neighbor?"}</li>
                                            <li>{"Sufis, Bhakti Sants, and Bauls though different in theology all emphasized personal virtues over rituals to experience the Divine?"}</li>
                                        </ul>
                                    </div>

                                    // <!-- Cultural Experience Section -->
                                    <div class="content">
                                        <p class="subtitle has-text-weight-semibold">{"Cultural Experience"}</p>
                                        <p>{"Experience the cultural heritage of the Indian subcontinent as we embark on a journey through time, tracing the arrival of Sufis from Persia, and delving into the profound influence of Hafiz’s poetic lyricism, Kabir's humanistic philosophy, and Lalan’s folk wisdom on Tagore. Witness top local artists from Maryland and Virginia bring this rich cultural tapestry to life."}</p>
                                    </div>
                                </div>
                            </article>
                    </div>
                </div>
            </div>
        </div>
    }
}
