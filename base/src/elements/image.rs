use yew::prelude::*;

use crate::props::{ImageSize, Rounded};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub size: Option<ImageSize>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub src: Option<AttrValue>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// A container for responsive images - [reference](https://bulma.io/documentation/elements/image/)
#[function_component(Image)]
pub fn image(props: &Props) -> Html {
    let class = classes!("image", props.class.clone(), props.size);

    let loaded = use_state(|| false);

    let onload = {
        let cloned = loaded.clone();
        Callback::from(move |_| cloned.set(true))
    };

    let display = ["display:none", ""];

    html! {
        <figure style={props.style.clone()} {class}>
            <img class={ classes!(props.rounded) } src={ props.src.clone() } {onload} style={display[!*loaded as usize]}/>
            <img class={ classes!(props.rounded) } src={ props.placeholder.clone() } style={display[*loaded as usize]} />
        </figure>
    }
}
