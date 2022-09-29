use yew::prelude::*;

use base::elements::Icon;
use base::form::{Help, Label};
use base::props::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub label: Option<AttrValue>,

    #[prop_or_default]
    pub optional: bool,

    #[prop_or_default]
    pub success: bool,

    #[prop_or_default]
    pub help: Option<AttrValue>,

    #[prop_or_default]
    pub icon_right: Option<AttrValue>,

    #[prop_or_default]
    pub icon_left: Option<AttrValue>,
}

#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let help = match &props.help {
        Some(help) => html! { <Help color={Color::Danger}> {help.clone()} </Help> },
        None => html! {},
    };

    let label = match (&props.label, props.optional) {
        (Some(label), true) => html! {<Label> {label.clone()} {"-"} <i>{"Optional"}</i></Label>},
        (Some(label), false) => html! {<Label> {label.clone()} </Label>},
        _ => html! {},
    };

    let right = match &props.icon_right {
        Some(right) => html! {<Icon icon={right.clone()} class="is-right"/>},
        None => html! {},
    };

    let left = match &props.icon_left {
        Some(left) => html! {<Icon icon={left.clone()} class="is-left"/>},
        None => html! {},
    };

    let class = classes!(
        "control",
        props.icon_right.as_ref().map(|_| "has-icons-right"),
        props.icon_left.as_ref().map(|_| "has-icons-left")
    );

    let enclose = |inner: Html, context: Color| html! { <ContextProvider<Color> {context}> {inner} </ContextProvider<Color>> };

    let body = match (&props.help, props.success) {
        (Some(_), _) => enclose(html! { for props.children.iter() }, Color::Danger),
        (_, true) => enclose(html! { for props.children.iter() }, Color::Success),
        (None, _) => html! { for props.children.iter() },
    };

    html! {
        <div class={classes!("field", props.class.clone())}>
            { label }
            <div {class}>
                { right }
                { left }
                { body }
            </div>
            { help }
        </div>
    }
}
