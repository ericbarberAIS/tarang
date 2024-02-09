use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<&'static str>, // URLs of the images
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let active_image = use_state(|| 0usize);

    // let on_next = {
    //     let active_image = active_image.clone();
    //     let images_len = props.images.len();
    //     Callback::from(move |_| {
    //         active_image.set((*active_image + 1) % images_len);
    //     })
    // };

    let on_prev = {
        let active_image = active_image.clone();
        let images_len = props.images.len();
        Callback::from(move |_| {
            active_image.set(if *active_image == 0 {
                images_len - 1
            } else {
                *active_image - 1
            });
        })
    };

    html! {
        <div class="section container">
            <div class="carousel">
                <div class="carousel-images">
                    <img onclick={on_prev} src={props.images[*active_image]} />
                </div>
                // <button onclick={on_prev}>{ "Previous" }</button>
                // <button onclick={on_next}>{ "Next" }</button>
            </div>
        </div>
    }
}
