use yew::prelude::*;

use crate::props::{Color, Light};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub delete: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
#[function_component(Notification)]
pub fn notification(props: &Props) -> Html {
    let classes = classes!(
        "notification",
        props.class.clone(),
        props.color,
        props.light
    );

    let button = match props.delete == Callback::noop() {
        false => html! {<button class="delete" onclick={cb.reform(|_| ())}></button>},
        true => html! {},
    };

    html! {
        <div style={props.style.clone()} class={classes}>
            { button }
            { for props.children.iter() }
        </div>
    }
}
