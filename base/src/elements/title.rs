use yew::prelude::*;

use crate::props::{HeaderSize, Spaced};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub spaced: Spaced,

    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is3);
    let classes = classes!("title", &props.extra, size, props.spaced);

    html! {
        <p class={classes}>
            { for props.children.iter() }
        </p>
    }
}
