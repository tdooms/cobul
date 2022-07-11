use base::props::{Color, Size};
use yew::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: &'static str,
    pub checked: bool,

    pub label: String,
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub label_left: bool,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<Size>,

    #[prop_or_default]
    pub thin: bool,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub outlined: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Switch)]
pub fn switch(props: &Props) -> Html {
    let class = classes!(
        "switch",
        props.class.clone(),
        props.label_left.then(|| "is-rtl"),
        props.color,
        props.size,
        props.thin.then(|| "is-thin"),
        props.rounded.then(|| "is-rounded"),
        props.outlined.then(|| "is-outlined"),
    );

    let checked = props.checked;

    html! {
        <>
        <input id={props.id} {class} type="checkbox" checked=true name={props.id}
            disabled={props.disabled} onchange={props.onchange.reform(move |_| !checked)}/>
        <label for={props.id}> {props.label.clone()} </label>
        </>
    }
}
