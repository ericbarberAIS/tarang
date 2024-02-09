use web_sys::window;
use yew::prelude::*; // For accessing the global window object to manipulate scroll position

#[derive(Properties, PartialEq)]
pub struct AnchorLinkProps {
    pub target: String,
    pub label: String,
    pub class: String,
}

#[function_component(AnchorLink)]
pub fn anchor_link(props: &AnchorLinkProps) -> Html {
    let onclick = {
        let target = props.target.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default(); // Prevent the default anchor behavior
            if let Some(element) = window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id(&target))
            {
                element.scroll_into_view(); // Scroll the target element into view
            }
        })
    };

    html! {
        // <a href={props.target.clone()} {onclick}>{ &props.label }</a>
        <a class={props.class.clone()} href={props.target.clone()} {onclick}>{ &props.label }</a>
    }
}
