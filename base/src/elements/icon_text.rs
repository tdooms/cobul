use yew::prelude::*;

use crate::props::TextColor;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub style: String,
}

/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
#[function_component(IconText)]
pub fn icon_text(props: &Props) -> Html {
    let classes = classes!("icon-text", props.class.clone(), props.color);
    html! {<span style={props.style.clone()} class={classes}> { for props.children.iter() } </span>}
}
