use yew::prelude::*;

pub struct AboutUs;
impl Component for AboutUs {
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
                        <h1 class="title is-1">{ "About Us" }</h1>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image is-3by1">
                        <img alt="A random image for the input term 'sound'." src="https://source.unsplash.com/random/1200x400/?sound" />
                    </figure>
                </div>

                <div class="tile is-child hero">
                    <div class="hero-body container box">

                         <div id="about-us" class="hero-body">
                            <p>{"Welcome to Tarang, a dynamic and inclusive community dedicated to celebrating the rich cultural heritage of Eastern India. Our mission at Tarang, founded in 2024, is to unite people of Eastern Indian descent and enthusiasts alike, fostering a sense of community and connection through the enchanting realms of music and tradition."}</p><br/>
                            <p>{"At Tarang, we recognize the vibrant tapestry of Eastern Indian culture, characterized by its diverse languages, customs, and art forms. Central to our ethos is the captivating power of music, a universal language that transcends boundaries and brings us closer to our roots. Our organization serves as a beacon for those seeking to explore, express, and immerse themselves in the cultural identity of Eastern India."}</p><br/>
                            <p>{"We organize a variety of activities aimed at bridging generations and cultivating shared experiences. From classical music concerts and folk dance workshops to culinary feasts and storytelling sessions, our events celebrate the traditions that echo through time. Tarang also hosts cultural festivals, spotlighting the richness of Eastern Indian arts and providing members with opportunities to engage, learn, and contribute to our vibrant community."}</p><br/>
                            <p>{"Education is pivotal in our journey. Tarang offers classes and seminars on the history, philosophy, and practice of Eastern Indian music and arts. Catering to all skill levels, these educational initiatives encourage participants to delve into the depths of our heritage and foster a deeper understanding and appreciation of our culture."}</p><br/>
                            <p>{"Tarang is more than just an organization – it's a community of belonging. Here, members find support, inspiration, and camaraderie. We warmly welcome individuals of Eastern Indian descent and those who wish to explore and experience the wonders of our culture. Join us as we embark on this exciting journey of cultural discovery and celebration. Let's unite in keeping the melody of our shared heritage vibrant and resonant for generations to come."}</p><br/>
                            <p>{"Together at Tarang, we are not just preserving traditions; we are creating a legacy."}</p>
                        </div>
                    </div>
                </div>
                // <div class="tile is-child">
                //     <figure class="image is-3by1">
                //         <img alt="A random image for the input term 'sound'." src="https://source.unsplash.com/random/1200x400/?sound" />
                //     </figure>
                // </div>
            </div>
        }
    }
}
