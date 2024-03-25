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
                        <h1 class="title is-1 has-text-primary">{ "About Us" }</h1>
                    </div>
                </div>

                <div class="tile is-child">
                    <figure class="image">
                        <img alt="Performance from Tarang." src="/static/about/img/hero_image.jpg" />
                    </figure>
                </div>

                <div class="tile is-child hero">
                    <div class="hero-body container box">

                         <div id="about-us" class="hero-body">
                              <p>
                              <span class="has-text-primary">{"Welcome " }</span>
                              <span class="has-text-primary">{"to "}</span>
                              <span class="has-text-primary">{"Tarang,"}</span>
                            {" a dynamic and inclusive community dedicated to celebrating the rich cultural heritage of the Indian subcontinent. Our mission at Tarang, founded in 2023, is to culturally enrich the lives of the diaspora through intimate storytelling about the shared culture, and fostering a sense of community and connection through the enchanting realms of performing arts and tradition."}</p><br/>
                            <p>{"At Tarang, we recognize the rich heritage of the Indian subcontinent, characterized by its diverse customs, literature, culture, and art forms."}</p><br/>
                            <p>{"Central to our ethos is the captivating power of storytelling through performance arts, an universal language that transcends boundaries and brings us closer to better understanding the shared history. For example, Persian was an official language of the-then India for over 600 years which left an indelible impact on generations of scholars and writers in that region. Similarly, over millennia numerous scholars, saints, kings, and sultans in India helped shape extensively the history, culture, and traditions of its neighbors in innumerable ways. These fascinating stories are often wrapped inside college textbooks, conference proceedings, academic journals, and archived media records, and therefore inaccessible to the broader public. Tarang aims to highlight these less noticed but culturally important aspects of shared history in its programs and bring them alive for the audience through well-crafted storytelling that would appeal to all curious minds."}</p><br/>
                            <p>{"Our organization intends to serve as a beacon for those seeking to explore, express, and immerse themselves in this shared cultural history."}</p><br/>
                            <p>{"We aim to organize cultural events every year in the Greater Washington DC Metro Area and promote local talents from the diaspora. These showcase events will not only cultivate shared experiences and traditions that echo through time but also help raise funds for local charities."}</p><br/>
                            <p>{"In lieu of purchasing tickets to a Tarang event, the attendees would be expected to contribute directly to designated charities announced on the Tarang website. 100% of patrons’ contributions would therefore go to charities pre- selected and vetted by the Tarang team prior to each event."}</p><br/>
                            <p>{"Over time, Tarang will aim to provide members with opportunities to engage, learn, and contribute to the vibrant community. Spreading awareness and facilitating deeper understanding of shared cultural history is pivotal to our journey."}</p><br/>
                            <p>{"We warmly welcome interested individuals who wish to explore, perform, or experience the wonders of the Indic culture. Join us as we embark on this exciting journey of cultural discovery and celebration. Let's unite in keeping the melody of the shared heritage vibrant and resonant for generations to come."}</p><br/>
                            <p>{"Together at Tarang, we are not just preserving traditions; we are creating a legacy."}</p>
                        </div>
                    </div>
                </div><br/><br/>
            </div>
        }
    }
}
