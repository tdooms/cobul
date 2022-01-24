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
    pub src: Option<String>,
}

/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
#[function_component(Image)]
pub fn image(props: &Props) -> Html {
    let classes = classes!("image", props.class.clone(), props.size);

    html! {
        <figure class={classes}>
            <img class={ classes!(props.rounded) } src={ props.src.clone() } />
        </figure>
    }
}
