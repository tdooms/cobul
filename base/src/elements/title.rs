use yew::prelude::*;

use crate::props::{HeaderSize, Spaced};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub spaced: Spaced,

    #[prop_or_default]
    pub size: Option<HeaderSize>,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is3);
    let classes = classes!("title", props.class.clone(), size, props.spaced);

    html! {
        <p style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </p>
    }
}
