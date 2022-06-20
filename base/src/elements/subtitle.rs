use yew::prelude::*;

use crate::props::HeaderSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Option<HeaderSize>,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Subtitle)]
pub fn subtitle(props: &Props) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is5);
    let classes = classes!("subtitle", props.class.clone(), size);

    html! {
        <p style={props.style.clone()} class={classes}>
            { for props.children.iter() }
        </p>
    }
}
