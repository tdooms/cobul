use base::props::{Color, Size};
use rand::Rng;
use yew::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub checked: bool,

    pub label: String,
    pub change: Callback<bool>,

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
    let id = use_state(|| rand::thread_rng().gen::<u64>().to_string());

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
    let onchange = props.change.reform(move |_| !checked);

    html! {
        <>
        <input id={(*id).clone()} {class} type="checkbox" {checked} disabled={props.disabled} {onchange}/>
        <label for={(*id).clone()}> {props.label.clone()} </label>
        </>
    }
}
