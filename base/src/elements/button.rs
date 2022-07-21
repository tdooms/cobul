use yew::prelude::*;

use crate::props::{
    Active, Color, Focused, Fullwidth, Hidden, Hovered, Inverted, Light, Loading, Outlined,
    Rounded, Selected, Size, Static,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub hidden: Hidden,

    #[prop_or_default]
    pub outlined: Outlined,

    #[prop_or_default]
    pub inverted: Inverted,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub selected: Selected,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub r#static: Static,

    #[prop_or_default]
    pub tooltip: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,
}

/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let class = classes!(
        "button",
        props.hidden,
        props.outlined,
        props.light,
        props.inverted,
        props.rounded,
        props.loading,
        props.fullwidth,
        props.selected,
        props.color,
        props.size,
        props.hovered,
        props.focussed,
        props.active,
        props.r#static,
        props.class.clone()
    );

    let onclick = props.onclick.reform(|_| ());

    let Props {
        style,
        disabled,
        tooltip,
        ..
    } = props.clone();

    html! {
        <button {style} {class} {onclick} {disabled} data-tooltip={tooltip}>
            { for props.children.iter() }
        </button>
    }
}
