use yew::prelude::*;

use cobul_props::general::{
    Active, Disabled, Focused, Fullwidth, Hidden, Hovered, Inverted, Light, Loading,
    Outlined, Rounded, Selected, Static,
};
use cobul_props::{Color, Size};

#[derive(Properties, Clone, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub click: Callback<()>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub tooltip: Option<AttrValue>,

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
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    #[prop_or_default]
    pub active: Active,

    #[prop_or_default]
    pub statik: Static,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
}

/// The classic button, in different colors, sizes, and states - [reference](https://bulma.io/documentation/elements/button/)
///
/// Properties:
/// - `click: Callback<()>` &nbsp; Callback for button click
/// - `size: Option<Size>`
/// - `color: Option<Color>`
/// - `tooltip: Option<AttrValue>` &nbsp; The text tooltip on hover
/// - `hidden: Hidden`
/// - `outlined: Outlined`
/// - `inverted: Inverted`
/// - `rounded: Rounded`
/// - `light: Light`
/// - `loading: Loading`
/// - `disabled: Disabled`
/// - `fullwidth: Fullwidth`
/// - `selected: Selected`
/// - `hovered: Hovered`
/// - `focussed: Focused`
/// - `active: Active`
/// - `statik: Static`
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let hidden = use_context::<Hidden>();
    let outlined = use_context::<Outlined>();
    let light = use_context::<Light>();
    let inverted = use_context::<Inverted>();
    let rounded = use_context::<Rounded>();
    let loading = use_context::<Loading>();
    let fullwidth = use_context::<Fullwidth>();
    let selected = use_context::<Selected>();
    let color = use_context::<Color>();
    let size = use_context::<Size>();
    let hovered = use_context::<Hovered>();
    let focussed = use_context::<Focused>();
    let active = use_context::<Active>();
    let statik = use_context::<Static>();

    let class = classes!(
        "button",
        props.hidden.or(hidden),
        props.outlined.or(outlined),
        props.light.or(light),
        props.inverted.or(inverted),
        props.rounded.or(rounded),
        props.loading.or(loading),
        props.fullwidth.or(fullwidth),
        props.selected.or(selected),
        props.color.or(color),
        props.size.or(size),
        props.hovered.or(hovered),
        props.focussed.or(focussed),
        props.active.or(active),
        props.statik.or(statik),
        props.class.clone()
    );

    let onclick = props.click.reform(|_| ());
    let style = props.style.clone();

    html! {
        <button {style} {class} {onclick} disabled={props.disabled.0} data-tooltip={props.tooltip.clone()}>
            { for props.children.iter() }
        </button>
    }
}
