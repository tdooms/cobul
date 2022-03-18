use yew::prelude::*;

use crate::props::{Color, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub ondelete: Option<Callback<()>>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,
}

/// [https://bulma.io/documentation/components/message/](https://bulma.io/documentation/components/message/)
#[function_component(Message)]
pub fn message(props: &Props) -> Html {
    let header = match (props.header.clone(), props.ondelete.clone()) {
        (Some(html), Some(ondelete)) => html! {
            <div class="message-header"> {html}
                <button class="delete" aria-label="delete" onclick={ondelete.reform(|_| ())}></button>
            </div>
        },
        (None, Some(ondelete)) => html! {
            <div class="message-header">
                <button class="delete" aria-label="delete" onclick={ondelete.reform(|_| ())}></button>
            </div>
        },
        (Some(html), None) => html! {<div class="message-header"> {html} </div> },
        (None, None) => html! {},
    };

    html! {
        <article class={classes!("message", props.color, props.size)}>
            { header }
            <div class="message-body">
                { for props.children.iter() }
            </div>
        </article>
    }
}
