use yew::prelude::*;

use crate::{Help, Icon, Label};
use cobul_props::{Align, Color, Size};
use cobul_props::general::{Addons, Grouped, GroupedMultiline};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub enter: Callback<()>,

    #[prop_or_default]
    pub grouped: Grouped,

    #[prop_or_default]
    pub multiline: GroupedMultiline,

    #[prop_or_default]
    pub addons: Addons,

    #[prop_or_default]
    pub align: Option<Align>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub label: Option<AttrValue>,

    #[prop_or_default]
    pub help: Option<AttrValue>,

    #[prop_or_default]
    pub right: Option<AttrValue>,

    #[prop_or_default]
    pub left: Option<AttrValue>,
}

#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let help = match &props.help {
        Some(help) => html! { <Help color={props.color.clone()}> {help.clone()} </Help> },
        None => html! {},
    };

    let label = match &props.label {
        Some(label) => html! { <Label> {label.clone()} </Label> },
        None => html! {},
    };

    let right = match &props.right {
        Some(right) => html! {<Icon icon={right.clone()} size={props.size} class="is-right"/>},
        None => html! {},
    };

    let left = match &props.left {
        Some(left) => html! {<Icon icon={left.clone()} size={props.size} class="is-left"/>},
        None => html! {},
    };

    let align = match props.align {
        Some(Align::Centered) => "has-addons-centered",
        Some(Align::Right) => "has-addons-right",
        Some(Align::Left) => "has-addons-left",
        None => "",
    };

    let class = classes!(
        "control",
        props.right.as_ref().map(|_| "has-icons-right"),
        props.left.as_ref().map(|_| "has-icons-left"),
    );

    let first = match props.children.iter().next() {
        Some(first) => html! { <div {class}> {first} {right} {left} </div> },
        None => return html! {},
    };
    let first = match props.size {
        Some(context) => html! { <ContextProvider<Size> {context}> { first } </ContextProvider<Size>>},
        None => html! { first },
    };
    let first = match props.color {
        Some(context) => html! { <ContextProvider<Color> {context}> { first } </ContextProvider<Color>>},
        None => html! { first },
    };

    let rest = props.children.iter().skip(1).map(|child| html! {
        <div> { child } </div>
    });

    let onkeypress = props.enter.reform(|e: KeyboardEvent| {
        if e.key() == "Enter" { e.prevent_default() }
    });

    let class = classes!("field",
        props.class.clone(),
        props.multiline,
        props.addons,
        props.grouped,
        align
    );

    html! {
        <div {class} {onkeypress}>
            { label }
            { first }
            { for rest }
            { help }
        </div>
    }
}
