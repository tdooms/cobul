use yew::prelude::*;

use crate::properties::{Size, TextColor};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub icon: String,

    #[prop_or_default]
    pub extra: String,

    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<()>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<TextColor>,
}

/// A container for any type of icon font.
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[function_component(Icon)]
pub fn icon(props: &Props) -> Html {
    let classes = classes!("icon", &props.extra, props.size, props.color);
    let onclick = props.onclick.reform(|_| ());

    html! {
        <span class={classes} onclick={onclick}>
            <i class={props.icon.clone()}> </i>
        </span>
    }
}
