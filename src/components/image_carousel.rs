use web_sys::MouseEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<&'static str>, // URLs of the images
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    let active_image = use_state(|| 0usize);
    let is_dialog_open = use_state(|| false);
    // This state is not necessary if you always want to show the first image as the background
    // Keeping it for demonstration purposes
    let background_image_index = use_state(|| 0usize); // Always points to the first image

    let on_next = {
        let active_image = active_image.clone();
        let images_len = props.images.len();
        Callback::from(move |_: MouseEvent| {
            active_image.set((*active_image + 1) % images_len);
        })
    };

    let on_prev = {
        let active_image = active_image.clone();
        let images_len = props.images.len();
        Callback::from(move |_: MouseEvent| {
            active_image.set(if *active_image == 0 {
                images_len - 1
            } else {
                *active_image - 1
            });
        })
    };

    let toggle_dialog = {
        let is_dialog_open = is_dialog_open.clone();
        Callback::from(move |_: MouseEvent| {
            is_dialog_open.set(!*is_dialog_open);
        })
    };

    let dialog_content = if *is_dialog_open {
        html! {
            <div class="modal is-active">
                <div class="modal-background" onclick={toggle_dialog.clone()}></div>
                <div class="modal-content">
                    <p class="image">
                        <img src={props.images[*active_image]} onclick={toggle_dialog.clone()} />
                    </p>
                    // Navigation buttons container with flexbox
                    <div class="buttons-container" style="display: flex; justify-content: space-between; margin-top: 10px;">
                        <button onclick={on_prev.clone()} class="button is-light is-small">{"Previous"}</button>
                        <button onclick={on_next.clone()} class="button is-light is-small">{"Next"}</button>
                    </div>
                </div>
                <button class="modal-close is-large" aria-label="close" onclick={toggle_dialog.clone()}></button>
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div class="section container">
            <div class="carousel" onclick={toggle_dialog}>
                <div class="carousel-images">
                    // Always display the first image as the background
                    <img src={props.images[*background_image_index]} />
                </div>
            </div>
            {dialog_content}
        </div>
    }
}
