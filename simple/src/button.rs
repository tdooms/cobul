use yew::prelude::*;

use cobul_base::elements;
use cobul_base::props::{
    Active, Color, Disabled, Focused, Fullwidth, Hidden, Hovered, Inverted, Light, Loading,
    Outlined, Rounded, Selected, Size, Static,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Option<Size>,

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
    pub disabled: Disabled,

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
    pub statik: Static,

    #[prop_or_default]
    pub tooltip: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub icon: Option<AttrValue>,

    #[prop_or_default]
    pub text: Option<AttrValue>,
}

/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let inner = match (props.icon.clone(), props.text.clone()) {
        (None, None) => html! {},
        (None, Some(text)) => html! {text},
        (Some(icon), None) => html! { <elements::Icon {icon} /> },
        (Some(icon), Some(text)) => html! {<> <elements::Icon {icon} /> <span> {text} </span> </>},
    };

    html! {
        <elements::Button style={props.style.clone()} class={props.class.clone()} click={props.click.clone()}
        disabled={props.disabled} tooltip={props.tooltip.as_ref().map(ToString::to_string)} hidden={props.hidden} outlined={props.outlined}
        light={props.light} inverted={props.inverted} rounded={props.rounded} loading={props.loading}
        fullwidth={props.fullwidth} selected={props.selected} color={props.color} size={props.size}
        hovered={props.hovered} focussed={props.focussed} active={props.active} statik={props.statik}>
            {inner}
        </elements::Button>
    }
}
