use base::elements;
use base::props::{Color, Size};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub hidden: bool,

    #[prop_or_default]
    pub outlined: bool,

    #[prop_or_default]
    pub inverted: bool,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub light: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub hovered: bool,

    #[prop_or_default]
    pub focussed: bool,

    #[prop_or_default]
    pub active: bool,

    #[prop_or_default]
    pub r#static: bool,

    #[prop_or_default]
    pub tooltip: Option<String>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub icon: Option<String>,

    #[prop_or_default]
    pub text: String,
}

/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let inner = match props.icon.clone() {
        None => html! {props.text.clone()},
        Some(icon) => html! {<> <elements::Icon {icon} /> <span> {props.text.clone()} </span> </>},
    };

    html! {
        <elements::Button style={props.style.clone()} class={props.class.clone()} click={props.click.clone()}
        disabled={props.disabled} tooltip={props.tooltip.clone()} hidden={props.hidden} outlined={props.outlined}
        light={props.light} inverted={props.inverted} rounded={props.rounded} loading={props.loading}
        fullwidth={props.fullwidth} selected={props.selected} color={props.color} size={props.size}
        hovered={props.hovered} focussed={props.focussed} active={props.active} r#static={props.r#static}>
            {inner}
        </elements::Button>
    }
}
