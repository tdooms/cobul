use yew::prelude::*;

use crate::properties::{Size, TextColor};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<TextColor>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(IconText)]
pub fn icon_text(props: &Props) -> Html {
    let classes = classes!("icon-text", &props.extra, props.color);
    html! {<span class={classes}> { for props.children.iter() } </span>}
}
