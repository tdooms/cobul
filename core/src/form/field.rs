use yew::prelude::*;

use cobul_raw::elements::Icon;
use cobul_raw::form::{Help, Label};
use cobul_props::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub enter: Callback<()>,

    #[prop_or_default]
    pub class: Classes,

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
        Some(help) => html! { <Help color={Color::Danger}> {help.clone()} </Help> },
        None => html! {},
    };

    let label = match &props.label {
        Some(label) => html! { <Label> {label.clone()} </Label> },
        None => html! {},
    };

    let right = match &props.right {
        Some(right) => html! {<Icon icon={right.clone()} class="is-right"/>},
        None => html! {},
    };

    let left = match &props.left {
        Some(left) => html! {<Icon icon={left.clone()} class="is-left"/>},
        None => html! {},
    };

    let class = classes!(
        "control",
        props.right.as_ref().map(|_| "has-icons-right"),
        props.left.as_ref().map(|_| "has-icons-left")
    );

    let onkeypress = props.enter.reform(|e: KeyboardEvent| {
        if e.key() == "Enter" {
            e.prevent_default()
        }
    });

    html! {
        <div class={classes!("field", props.class.clone())} {onkeypress}>
            { label }
            <div {class}>
                { right }
                { left }
                { for props.children.iter() }
            </div>
            { help }
        </div>
    }
}