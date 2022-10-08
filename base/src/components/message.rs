use yew::prelude::*;

use crate::props::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub delete: Callback<()>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// Colored message blocks, to emphasize part of your page - [reference](https://bulma.io/documentation/components/message/)
#[function_component(Message)]
pub fn message(props: &Props) -> Html {
    let header = match (props.header.clone(), props.delete == Callback::noop()) {
        (Some(html), true) => html! {
            <div class="message-header">
                {html}
                <button class="delete" aria-label="delete" onclick={props.delete.reform(|_| ())}></button>
            </div>
        },
        (None, true) => html! {
            <div class="message-header">
                <button class="delete" aria-label="delete" onclick={props.delete.reform(|_| ())}></button>
            </div>
        },
        (Some(html), false) => html! { <div class="message-header"> {html} </div> },
        (None, false) => html! {},
    };

    html! {
        <article style={props.style.clone()} class={classes!("message", props.color, props.size)}>
            { header }
            <div class="message-body">
                { for props.children.iter() }
            </div>
        </article>
    }
}
