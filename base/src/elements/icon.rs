use yew::prelude::*;

use crate::props::{Size, TextColor};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub icon: String,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let classes = classes!("icon", props.class.clone(), props.size, props.color);
    let onclick = props.onclick.reform(|_| ());

    html! {
        <span style={props.style.clone()} class={classes} onclick={onclick}>
            <i class={props.icon.clone()}> </i>
        </span>
    }
}
